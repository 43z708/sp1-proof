use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// actix_web::rt::task を利用
use actix_web::rt::task;

use sp1_sdk::{ProverClient, SP1Stdin, HashableKey};

#[derive(Deserialize, Debug)]
struct GameResult {
    final_score: u32,
    enemy_counts: HashMap<String, u32>,
}

#[derive(Serialize)]
struct ProofData {
    proof: String,         // proof のバイト列を hex 表記
    public_inputs: String, // 公開入力（Vec<u8> へ変換済み）
    vkey_hash: String,     // 検証鍵のハッシュ
    mode: String,          // 使用した証明モード ("groth16" 等)
}

#[post("/generate-proof")]
async fn generate_proof(item: web::Json<GameResult>) -> impl Responder {
    println!("DEBUG: Received game result: {:?}", item);
    let game_result = item.into_inner();

    // SP1 の入力として、最終スコアと各敵撃破数を順序固定で書き込みます
    let mut stdin = SP1Stdin::new();
    println!("DEBUG: Writing final_score: {}", game_result.final_score);
    stdin.write(&game_result.final_score);

    let mut enemy_keys: Vec<_> = game_result.enemy_counts.keys().cloned().collect();
    enemy_keys.sort();
    println!("DEBUG: Sorted enemy keys: {:?}", enemy_keys);
    for key in enemy_keys {
        if let Some(count) = game_result.enemy_counts.get(&key) {
            println!("DEBUG: Writing enemy count for key {}: {}", key, count);
            stdin.write(count);
        }
    }

    // 証明生成用の回路 ELF ファイルを読み込む
    // ※ プロジェクトルートに配置されている "game-proof.elf" を参照します（相対パスで指定）
    let game_elf: &[u8] = include_bytes!("../game-proof.elf");
    println!("DEBUG: Loaded game ELF ({} bytes)", game_elf.len());

    // ブロッキング操作を別スレッドで実行（spawn_blocking を使用）
    let blocking_result = task::spawn_blocking(move || -> Result<(Vec<u8>, String, Vec<u8>), Box<dyn std::error::Error + Send>> {
        println!("DEBUG: [spawn_blocking] Starting proof generation");
        let client = ProverClient::from_env();
        println!("DEBUG: [spawn_blocking] ProverClient initialized");
        let (pk, vk) = client.setup(game_elf);
        println!("DEBUG: [spawn_blocking] Setup completed");
        let proof = client.prove(&pk, &stdin).groth16().run()?;
        println!("DEBUG: [spawn_blocking] Proof generated successfully");
        let proof_bytes = proof.bytes().to_vec();
        let public_values_vec = proof.public_values.to_vec(); // SP1PublicValues -> Vec<u8> (仮の変換)
        let vkey_hash = vk.bytes32();
        println!(
            "DEBUG: [spawn_blocking] Proof details: proof_bytes ({} bytes), public_inputs ({} bytes), vkey_hash: {}",
            proof_bytes.len(),
            public_values_vec.len(),
            vkey_hash
        );
        Ok((proof_bytes, vkey_hash, public_values_vec))
    })
    .await
    .map_err(|e| format!("Blocking task join error: {:?}", e));

    let (proof_bytes, vkey_hash, public_values_vec) = match blocking_result {
        Ok(Ok(tuple)) => {
            println!("DEBUG: [spawn_blocking] Task completed successfully");
            tuple
        },
        Ok(Err(e)) => {
            println!("DEBUG: [spawn_blocking] Task error: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Proof generation failed: {:?}", e));
        }
        Err(e) => {
            println!("DEBUG: [spawn_blocking] Join error: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Blocking task join error: {:?}", e));
        }
    };

    let proof_data = ProofData {
        proof: hex::encode(proof_bytes),
        public_inputs: hex::encode(public_values_vec),
        vkey_hash,
        mode: "groth16".to_string(),
    };

    match serde_json::to_string(&proof_data) {
        Ok(json_str) => {
            println!("DEBUG: Successfully serialized proof JSON");
            HttpResponse::Ok().content_type("application/json").body(json_str)
        },
        Err(e) => {
            println!("DEBUG: Serialization error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Serialization error: {:?}", e))
        }
    }
}

#[actix_web::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    println!("Starting proof API server on 0.0.0.0:8080");
    HttpServer::new(|| App::new().service(generate_proof))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
