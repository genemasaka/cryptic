use yew::prelude::*;
use ethers::prelude::*;
use ethers::abi::Abi;
 use ethers::contract::Contract;
 
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

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Model {
			password: String::new(),
			encrypted_password: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Encrypt => {
				let provider = Provider::<Http>::try_from("https://goerli.infura.io/v3/16087e2d4b5247d589382c9038b12f12");
				let signer = LocalSigner::new("ac5bb51f6a3012f69e637f82fd2c24524149231162a31450d18bd375becfc7f8", provider.clone());
				let contract = Contract::new(provider, CONTRACT_ADDRESS, CONTRACT_ABI);

				let function = contract.function("encryptPassword".to_owned());
				let encoded_input = function.encode_input(self.password.clone());
				let tx = signer.sign(encoded_input);
				let hash = provider.send_transaction(tx).wait().unwrap();
				let receipt = provider.get_transaction_receipt(hash).wait().unwrap();
				let decoded_output = function.decode_output(receipt.output).unwrap();
				let encrypted_password = decoded_output[0].to_string();

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

	fn view(&self) -> Html {
		html! {
			<>
			<div>
				<input type="password",
				    value=&self.password,
				    oninput=|e| Msg::UpdatePassword(e.value),
				    placeholder="Enter password" />
				<button onclick=|_| Msg::Encrypt,>{Encrypt}</button>
				<br />
				<br />
				<p>{ &self.encrypted_password.clone().unwrap_or("Encrypted password should appear here".to_string()) }</p>
			</div>
			</>
		}
	}
}

fn main() {
	yew::start_app::<Model>();
}