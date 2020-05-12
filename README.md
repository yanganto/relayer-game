Relayer Game
---
[![Build Status](https://travis-ci.com/yanganto/s3handler.svg?branch=master)](https://travis-ci.com/yanganto/relayer-game)

`relayer-game` is a tool to simulate the optimized game for relayers in Darwinia Network. 
In ordre to ban the replay who lies, and also we want to help make thing finialize as soon as possible, 
this tool can easily to change three important equations and load from different sceneraio to simulate the relayer game.  

# Scenario
In this tool we assume the target chain is Ethereum, however you can simulate different chain by chaning parameters.
All the behavior of relayers, and the parameters are described a in a yaml file. 
You can easily load the scenario file to simulate the result. 
There are some example scenario files listed in [sceneraio](./sceneraio).

## General parameters
- `title ` (optional)
  - The title for this scenario will print on the console
- `F` (optional)
  - The block producing factor for darwinia / ethereum
  - For example: 2.0, that means that darwinia produce 2 blocks and ethereum produce 1 block.

## Specify functions type
- `wait_function`
  - Once a relayer submit a header and wait the time in blocks after the calculated value from wait function, 
    Darwinia network will deem this header is valided and become a best header.  
  - Current suport: intager number, linear   
  - For example: '10', that means a submit block will be deem to relayed and finialized after 10 darwinia blocks.
  - For exmaple: `wait_linear`, that means a submit block will wait according the linear function, and the parameters of function need to provide.

- `target_function `
  - Once there is disput on any header, the relayer should submit the next ethere block target as calculated.  
  - Current suport: half
  - For example: 'half', that means the next target will be `(submited_ethereum_block_height - relayed_ethereum_block_height) / 2`

- `fee_function`
  - The fee function will increase the fee to improve speed of the finality, and the cost of keeping lie will be enormous.  
  - Current suport: float number, linear   
  - For example: '10.0', that means the fee of each submit is always 10.0.

## Initalize status of Darwinia and Ethereum
surfix `d`: block difference between last block number relayed on Darwinia, surfix `e`: block difference between last related block number of Ethereum
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

## Parameters of Equation
The three function can use different equations, base on the function seting, following parameters of function should be filled.
- `[wait_linear]`
  - the linear eqiation for wait function
  - `waiting block = int(min(Wd * D, Md) + min(We * E, Me)) + C`
  - surfix `d`: block difference between last block number relayed on Darwinia, surfix `e`: block difference between last related block number of Ethereum
  - D is the distance in Darwinia chain
  - E is the distance in Ethereum chain
  - W is the weight for that protion
  - M is the maxium value for that protion

- `[fee_linear]`
  - the linear eqiation for fee function
  - `fee = min(W * E, M)) + C`
  - W is the weight for that protion
  - M is the maxium value for that protion

# Build & Run
This excutable is written in Rust, it can be easiy to build and run with cargo command.  
```
cargo build --release
```
then the binary will be placed in ./target/release, you can run this command with scenario file as  following command.  
```
./target/release/relayer-game scenario/basic.yml
```
Also, you can put `-v` option to see all status in each round of submitions
```
./target/release/relayer-game -v scenario/basic.yml
```




