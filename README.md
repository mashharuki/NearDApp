# NearDApp
Nearと連動するDApp開発用のリポジトリです。

## Near protocolとは
Near protocolは、シャーディングと呼ばれる技術を採用してるPoSパブリックプロコトルでETHなどと同じレイヤー1と呼ばれるブロックチェーン。  

また、Near protocolではETHと競合関係にありがならETHと互換性をもたせるために「Rainbowbridge」というサービスも提供してます。
  
これを利用することでNEAR⇔ETHの移動ができるようになり、ETHをメインに利用している人でも気軽にNEARネットワークを使用することが可能になりました。

### BlockExplorer

 [テストネットへのデプロイ例](https://explorer.testnet.near.org/transactions/GKapRwXm8uUs3Lf1jMNf9cPURdnoEg5G2uiDkE1brzwx)

### Near CLIのインストールコマンド
 `npm i -g near-cli`  
 `near`

### プロジェクトの雛形作成コマンド例
 `npx create-near-app --frontend=react --contract=rust --tests rust near-hotel-booking-dapp`  

 コマンドのオプションは必ず指定すること！！!!
 下記のようなエラーが出てくる。

 ```cmd
Arguments error
Run npx create-near-app without arguments, or use:
npx create-near-app <projectName> --contract rust|js|assemblyscript --frontend react|vanilla|none --tests js|rust
 ```

うまく行けば、下記のような内容が出力される。
```cmd
======================================================
👋 Welcome to NEAR! Learn more: https://docs.near.org/
🔧 Let's get your dApp ready.
======================================================
(NEAR collects anonymous information on the commands used. No personal information that could identify you is shared)


Creating a new NEAR dApp

======================================================
✅  Success! Created '/Users/harukikondo/git/NearDApp/near-hotel-booking-dap'
   with a smart contract in Rust and a frontend template in React.js.
🦀 If you are new to Rust please visit https://www.rust-lang.org 

  Your next steps:
   - Navigate to your project:
         cd /Users/harukikondo/git/NearDApp/near-hotel-booking-dap
   - Install all dependencies
         npm run deps-install
   - Test your contract in NEAR SandBox:
         npm test
   - Deploy your contract to NEAR TestNet with a temporary dev account:
         npm run deploy
   - Start your frontend:
         npm start
```

### テスト実行方法

`npm test`

```cmd
 Passed ✅ gets default message
    Passed ✅ changes message
```

### コントラクトをビルドする方法2

`cd contract`  
`cargo build --target wasm32-unknown-unknown --release`  

ビルドファイルが生成されているか確認する方法  
`ls ./target/wasm32-unknown-unknown/release | grep hotel_booking.wasm`  

Near Walletへログインするコマンド  

`near login`  

サブアカウントを作成するコマンド  
`near create-account contract.dev-1660204085773-49134722844982 --masterAccount dev-1660204085773-49134722844982 --initialBalance 5`   

スマートコントラクトをデプロイするコマンド  
`near deploy --wasmFile target/wasm32-unknown-unknown/release/hotel_booking.wasm --accountId contract.dev-1660204085773-49134722844982`  

うまく行けば下記のような内容が出力される。

```cmd
Starting deployment. Account id: contract.dev-1660204085773-49134722844982, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: target/wasm32-unknown-unknown/release/hotel_booking.wasm
Transaction Id 5hr8mjNtk2gwiDp4qezbV3dksQgYk8WRXV5fCLQ1PSri
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/5hr8mjNtk2gwiDp4qezbV3dksQgYk8WRXV5fCLQ1PSri
Done deploying to contract.dev-1660204085773-49134722844982
```

スマートコントラクトのメソッドをコマンドで呼び出す方法(viewメソッド)   

`near view contract.dev-1660204085773-49134722844982 get_rooms_registered_by_owner '{"owner_id": "dev-1660204085773-49134722844982"}' --accountId dev-1660204085773-49134722844982`

うまく行けば下記のように出力される。  

```cmd
View call: contract.dev-1660204085773-49134722844982.get_rooms_registered_by_owner({"owner_id": "dev-1660204085773-49134722844982"})
[]
```

Room情報を追加するためのcallメソッドを呼び出すコマンドは次の通り  

`near call contract.dev-1660204085773-49134722844982 add_room_to_owner '{"name": "Sun Room", "image": "https://ibb.co/kgNm4jP", "beds": 1, "description": "This is Sun room.", "location": "Tokyo", "price": "1000000000000000000000000"}' --accountId dev-1660204085773-49134722844982`  

成功していれば、viewメソッドを呼び出した時に次の内容が出力される。

```cmd
View call: contract.dev-1660204085773-49134722844982.get_rooms_registered_by_owner({"owner_id": "dev-1660204085773-49134722844982"})
[
  {
    name: 'Sun Room',
    image: 'https://ibb.co/kgNm4jP',
    beds: 1,
    description: 'This is Sun room.',
    location: 'Tokyo',
    price: '1000000000000000000000000',
    status: 'Available'
  }
]
```

existsメソッドを呼び出すコマンド  

`near view contract.dev-1660204085773-49134722844982 exists '{"owner_id": "dev-1660204085773-49134722844982", "room_name": "Sun Room"}' --accountId dev-1660204085773-49134722844982`

### テストネットへのデプロイ方法

`npm run deploy`

```cmd
 near-hotel-booking-dap@1.0.0 deploy
> npm run build:contract && cd contract && near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm


> near-hotel-booking-dap@1.0.0 build:contract
> cd contract && rustup target add wasm32-unknown-unknown && cargo build --all --target wasm32-unknown-unknown --release

info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
    Finished release [optimized] target(s) in 0.88s
Please help us to collect data on near-cli usage to improve developer experience. 
We will never send private information. We collect which commands are run with attributes, your account ID, and your country
Note that your account ID and all associated on-chain transactions are already being recorded on public blockchain. 

Would you like to opt in (y/n)? y
Starting deployment. Account id: dev-1660204085773-49134722844982, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./target/wasm32-unknown-unknown/release/hello_near.wasm
Transaction Id GKapRwXm8uUs3Lf1jMNf9cPURdnoEg5G2uiDkE1brzwx
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/GKapRwXm8uUs3Lf1jMNf9cPURdnoEg5G2uiDkE1brzwx
Done deploying to dev-1660204085773-49134722844982
```

### フロントエンド起動方法

`npm start`  

<img src="./assets/imgs/start.png">

#### 参考文献
1. [Create Near AppのNPMページ](https://www.npmjs.com/package/create-near-app?activeTab=readme)
2. [BlockExplorer](https://explorer.testnet.near.org/)
3. [Nearprotocolの開発者ドキュメント](https://docs.near.org/develop/quickstart-guide)
4. [Near SDK](https://docs.rs/near-sdk/latest/near_sdk/)
5. [【NEAR編】NEAR Walletの作成から購入方法。Rainbow bridgeでのブリッジ方法も徹底解説](https://koji-toku.com/near-wallet-bridge)
6. [BlockExplorer(mashharuki.testnet)](https://wallet.testnet.near.org/profile/mashharuki.testnet)
7. [NearWalletPortal](https://wallet.near.org/)
8. [NearUniversityJapan](https://near-university-japan.super.site/)