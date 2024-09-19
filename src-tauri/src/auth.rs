use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
#[allow(non_snake_case)]
struct AuthRequest {
    email: String,
    password: String,
    returnSecureToken: bool,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct AuthResponse {
    idToken: String,
}

pub async fn authenticate_user(email: &str, password: &str) -> Result<String, Box<dyn Error>> {
    // URL da API de autenticação do Firebase
    let firebase_api_key = "AIzaSyAba2N4miuwngWBOmBUa34YxCtacbFg_4E";
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        firebase_api_key
    );

    // Criação do cliente HTTP
    let client = Client::new();

    // Criação do corpo da requisição
    let auth_request = AuthRequest {
        email: email.to_string(),
        password: password.to_string(),
        returnSecureToken: true,
    };

    // Envio da requisição POST para o Firebase
    let response = client
        .post(&url)
        .json(&auth_request)
        .send()
        .await?;

    // Verificação da resposta e desserialização do JSON
    if response.status().is_success() {
        let auth_response: AuthResponse = response.json().await?;
        Ok(auth_response.idToken) // Retorna o idToken (JWT)
    } else {
        Err("Error".into())
    }
}
