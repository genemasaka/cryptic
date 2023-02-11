use yew::prelude::*;
use ethers_core::{
	abi::Abi,
	types::{Address, H256},
};
use ethers_contract::Contract;
use ethers_providers::{Middleware, Provider, Http};
use ethers_signers::LocalWallet;
use ethers_middleware::SignerMiddleware;
use std::convert::TryFrom;

const CONTRACT_ADDRESS: &str = "0x6b201D66eed55697f87F0dbD86C120497401f5e6";
const CONTRACT_ABI: Abi = serde_json::from_str(r#"
[
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "password",
				"type": "string"
			}
		],
		"name": "encryptPassword",
		"outputs": [
			{
				"internalType": "bytes32",
				"name": "",
				"type": "bytes32"
			}
		],
		"stateMutability": "pure",
		"type": "function"
	}
]
"#);

struct Model {
	password: String,
	encrypted_password: Option<String>,
}

enum Msg {
	Encrypt,
	UpdatePassword(String),
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		Model {
			password: String::new(),
			encrypted_password: None,
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Encrypt => {
				let provider = Provider::<Http>::try_from("https://goerli.infura.io/v3/16087e2d4b5247d589382c9038b12f12")
					.expect("could not instantiate HTTP Provider");
				let wallet: LocalWallet = "ac5bb51f6a3012f69e637f82fd2c24524149231162a31450d18bd375becfc7f8".parse()?;
				let mut client = SignerMiddleware::new(provider, wallet);

				let contract = Contract::new(CONTRACT_ADDRESS, CONTRACT_ABI, client);

				let call = contract.method::<_, H256>("encryptPassword".to_owned());
	
				self.encrypted_password = Some(encrypted_password);
				ConsoleService::log(&format!("Encrypted password: {}", encrypted_password));
			}
			Msg::UpdatePassword(password) => {
				self.password = password;
				self.encrypted_password = None;
			}
		}
		true
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<>
			<div>
				<input type="password"
				    value={&self.password}
				    oninput=|e| Msg::UpdatePassword(e.value)
				    placeholder="Enter password" />
				<button onclick={|_| Msg::Encrypt,>{Encrypt}}</button>
				<br />
				<br />
				<p>{ &self.encrypted_password.clone().unwrap_or("Encrypted password should appear here".to_string()) }</p>
			</div>
			</>
		}
	}
}

fn main() {
	yew::Renderer::<Model>::new().render();
}