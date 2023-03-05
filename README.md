# cryptic


This is a Yew web application that encrypts a user's password using a smart contract deployed on the Ethereum network. The application uses the yew framework for the frontend and the ethers-rs crate for the Ethereum backend.

## Usage

To run the application:

Clone the repository: git clone https://github.com/example/yew-eth-password-encryption.git
Change into the repository directory: cd yew-eth-password-encryption
Install the dependencies: cargo install
Build the application: cargo build
Run the application: cargo run
Open http://localhost:8000 in your web browser.
The application interface provides a password input field and a button to encrypt the password. When the user enters a password and clicks the "Encrypt" button, the application uses ethers-rs to call the encryptPassword function of the smart contract on the Ethereum network. The encrypted password is then displayed on the screen.

## Requirements

This application requires a running Ethereum node, such as geth or parity, or an Ethereum provider like Infura, to connect to the Ethereum network. The Ethereum provider URL is defined in the create method of the Model struct. You will need to change this to your own provider URL to use the application.

Additionally, you will need to have a wallet with some ETH to pay for the transaction fees. The wallet private key is defined in the update method of the Model struct. You will need to change this to your own wallet private key to use the application.

## License

This code is licensed under the MIT License.
