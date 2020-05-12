Relayer Game
---
[![Build Status](https://travis-ci.com/yanganto/s3handler.svg?branch=master)](https://travis-ci.com/yanganto/relayer-game)

`relayer-game` is a tool to simulate the optimized game for relayers in Darwinia Network. 
In order to ban the replay who lies, and also we want to help make thing finalize as soon as possible, 
this tool can easily to change three important equations and load from different scenario to simulate the relayer game.  

# Scenario
In this tool we assume the target chain is Ethereum, however you can simulate different chain by changing parameters.
All the behavior of relayers, and the parameters are described a in a yaml file. 
You can easily load the scenario file to simulate the result. 
There are some example scenario files listed in [scenario](./scenario).

## General parameters
- `title ` (optional)
  - The title for this scenario will print on the console
- `F` (optional)
  - The block producing factor for Darwinia / Ethereum
  - For example: 2.0, that means that Darwinia produce 2 blocks and Ethereum produce 1 block.

## Specify functions type
- `wait_function`
  - Once a relayer submit a header and wait the time in blocks after the calculated value from wait function, 
    Darwinia network will deem this header is valided and become a best header.  
  - Current support: integer number, linear   
  - For example: '10', that means a submit block will be deem to relayed and finalized after 10 Darwinia blocks.
  - For example: `wait_linear`, that means a submit block will wait according the linear function, and the parameters of function need to provide.

- `target_function `
  - Once there is dispute on any header, the relayer should submit the next Ethereum block target as calculated.  
  - Current support: half
  - For example: 'half', that means the next target will be `(submited_ethereum_block_height - relayed_ethereum_block_height) / 2`

- `fee_function`
  - The fee function will increase the fee to improve speed of the finality, and the cost of keeping lie will be enormous.  
  - Current support: float number, linear   
  - For example: '10.0', that means the fee of each submit is always 10.0.

## Initialize status of Darwinia and Ethereum
suffix `d`: block difference between last block number relayed on Darwinia, suffix `e`: block difference between last related block number of Ethereum
- `Dd`(optional)
  - the block difference between last block number relayed on Darwinia, 
- `De`(optional)
  - the block difference between last related block number of Ethereum

## Relayers' Chose
Relays will not always be honest, s/he may some times cheat or not response.  
The following parameters are used for relayers
- `[[relayers]]` 
  - `name` (optional)
    - if name is not provided, the relayer will be name with serial numbers
  - `choice`
    - relayer may be response as `H`(Honest), `L`(Lie), `N`(No response)
    - if the lengeht of chose are shorter than other relayers, it will be deem to no response.  

We assume there always is a good guy to relay the correct headers, and the guy will name `Darwinia`, 
and this relayer will be automatic add into the scenario when load from configure file, 
so please avoid to use this name for the relayer.

## Parameters of Equation
The three function can use different equations, base on the function setting, following parameters of function should be filled.
- `[wait_linear]`
  - the linear equation for wait function
  - `waiting block = int(min(Wd * D, Md) + min(We * E, Me)) + C`
  - suffix `d`: block difference between last block number relayed on Darwinia, suffix `e`: block difference between last related block number of Ethereum
  - D is the distance in Darwinia chain
  - E is the distance in Ethereum chain
  - W is the weight for that portion
  - M is the maximum value for that portion

- `[fee_linear]`
  - the linear equation for fee function
  - `fee = min(W * E, M) + C`
  - W is the weight 
  - M is the maximum value for the variable part

# Build & Run
This executable is written in Rust, it can be easily to build and run with cargo command.  
```
cargo build --release
```
then the binary will be placed in ./target/release, you can run this command with scenario file as  following command.  
```
./target/release/relayer-game scenario/basic.yml
```
Also, you can put `-v` option to see all status in each round of submit.
```
./target/release/relayer-game -v scenario/basic.yml
```

# Develop and Document
This project has document, you can use this command to show the document on browser.
`cargo doc --no-deps --open`
If you want to add more equation for different function, you can take a look the trait in [fee](./src/fee/mod.rs), [wait](./src/wait/mod.rs), [target](./src/target/mod.rs).
The `Equation` trait and `ConfigValidate` will guild you to add you customized equation. 
