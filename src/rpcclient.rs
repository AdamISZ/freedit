use toy_rpc::Client;
use autct::config::AutctConfig;
use autct::rpc::{RPCProofVerifyRequest, RPCProofVerifyResponse};
use autct::utils::APP_DOMAIN_LABEL;
use crate::CONFIG;

// import everything including the client stub generated by the macro
use autct::rpc::*;
use std::error::Error;
use base64::prelude::*;

pub async fn do_request(proof: String) -> Result<RPCProofVerifyResponse, Box<dyn Error>>{
    // convert proof string from base64 to binary
    let decoded_proof = BASE64_STANDARD.decode(proof.as_bytes());
    let proof = match decoded_proof {
        Err(e) => {return Err(e.into());},
        Ok(dp) => dp,
    };
    let mut autctcfg = AutctConfig::default();
    autctcfg.mode = Some("request".to_string());
    if !CONFIG.autct_rpc_port.is_none() {
        autctcfg.rpc_port = CONFIG.autct_rpc_port.clone();
    }
    if !CONFIG.autct_rpc_host.is_none() {
        autctcfg.rpc_host = CONFIG.autct_rpc_host.clone();
    }
    if !CONFIG.autct_keysets.is_none() {
        autctcfg.keysets = CONFIG.autct_keysets.clone();
    }
    if !CONFIG.autct_proof_file_str.is_none() {
        autctcfg.proof_file_str = CONFIG.autct_proof_file_str.clone();
    }
    if !CONFIG.autct_user_string.is_none() {
        autctcfg.user_string = CONFIG.autct_user_string.clone();
    }

    let rpc_port = autctcfg.rpc_port.clone().unwrap();
    let host: &str= &autctcfg.rpc_host.clone().unwrap();
    let port_str: &str = &rpc_port.to_string();
    let addr: String = format!("{}:{}", host, port_str);
    // request must specify *only one* context label, keyset;
    // which is fine if one running instance of freedit is only
    // supporting one (context, keyset pair).
    // TODO: support multiple by having the caller ask to use
    // a specific keyset
    let (mut cls, mut kss) = autctcfg.clone()
    .get_context_labels_and_keysets().unwrap();
    if kss.len() != 1 || cls.len() != 1 {
        return Err("You may only specify one context_label:keyset in the request".into())
    }
    let keyset = kss.pop().unwrap();
    let context_label = cls.pop().unwrap();
    println!("About to create the request with keyset: {:?}", keyset);
    let req: RPCProofVerifyRequest = RPCProofVerifyRequest {
        keyset,
        user_label: autctcfg.user_string.unwrap(),
        context_label,
        application_label: String::from_utf8(APP_DOMAIN_LABEL.to_vec()).unwrap(),
        proof,
    };
    let mut client = Client::dial(&addr).await.unwrap();
    client.set_default_timeout(std::time::Duration::from_secs(3));
    let result: RPCProofVerifyResponse = client
    .r_p_c_proof_verifier().verify(req)
    .await
    .unwrap();
    Ok(result)
}

