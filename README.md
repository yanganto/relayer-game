# Relayer Game
[![Build Status](https://travis-ci.com/yanganto/s3handler.svg?branch=master)](https://travis-ci.com/yanganto/relayer-game)

`refit` is a **re**layer **f**ee **i**nference **t**ool to simulate and optimized the game for relayers in Darwinia Network.  
In order to ban the replay who lies, and also we want to help make thing finalize as soon as possible, 
this tool can easily to change three important equations and load from different scenario to simulate the relayer game.  
Such that you can easily to tune tha parameters.

The `chain`, `relayer`, `challenger` in `/scenario/<model>`folder can read the scenario file and simulate with more detail.

## Scenario
In this tool we assume the target chain is Ethereum, however you can simulate different chain by changing parameters.
All the behavior of relayers, and the parameters are described a in a yaml file. 
You can easily load the scenario file to simulate the result. 
There are some example scenario files listed in [scenario](./scenario).

There are four different gaming models: `relayers-only`, `relayer-challenger`, `relayer-challengers`, and `relayers-take-over`.


In `relayers-only` mode and `relayers-take-over`, when someone is not accepted the block submitted by other relayer, he should submit the correct block to express his opinion.
In `relayer-challenger` mode and `relayer-challengers` mode, when someone is not accepted the block submitted by other relayer, he just put a challenge on chain to express his opinion.

Following table shows the main different between these mode.

| Rule \Mode                       | **Relayers-Only**  | **Relayer-Challenger** | **Relayer-Challengers** | **Relayers-Take-Over** |
|----------------------------------|--------------------|------------------------|-------------------------|------------------------|
| Only 1 relay submit blocks       |                    | :heavy_check_mark:     | :heavy_check_mark:      |                        |
| Allow challenger take over       |                    |                        | :heavy_check_mark:      | :heavy_check_mark:     |
| Once in participate all          | :heavy_check_mark: | :heavy_check_mark:     |                         |                        |
| Estoppel                         | :heavy_check_mark: |                        |                         |                        |
| Ensure correct 1st block overall | :heavy_check_mark: |                        |                         |                        |
| Versus mode                      | 1 vs many          | 1 vs 1                 | 1 vs many               | 1 vs many              |
| Possible results                 | slash/reward       | slash/reward           | slash/reward/return     | slash/reward/return    |

In all mode, the `sample function` will point out the next one or many blocks, the relayer(s) should submit on it.  
The `sample function` is subtle, and should different when the target chain using different consensus mechanism.  
There is a discussion **Sample function** section, but we will explain the mode with a general `half` sampling equation.

There is still a little possibility that the initial submit in from a valid branch chain,
so there is a stage two in the game, after that the blocks from the initial relayer are verified on chain.
There is a discussion in **Stage two** section.

If there is only one `[[challengers]]` in scenario file, the scenario will run in relayer-challenger mode.
The `scenario/challenger.yml` is a scenario for one relayer and one challenger, you may run it with `-v` option to know more about this.

If there is more than one `[[challengers]]` in scenario file, the scenario will run in relayer-challengers mode.
The `scenario/challengers.yml` is a scenario for one relayer with multiple challengers, you may run it with `-v` option to know more about this.


### relayers-only mode
There are 3 rules in relayers-only mode.
1. Anyone can relay a ethereum header on the darwinia chain
2. Confirmed or open a round of game
    - if there is only one block (or same blocks) over the challenge time, this block is confirmed.  (2-1)
    - else (there are different blocks in the same block height) the game is starts
      - everyone in the game should submit the block based on the `sample function` until closed (2-2-1)
        - Once the a block according sample function submit the next round of gamae is started, and become recursive in 2
    - anyone can get in the game in any round but need to participate until the game closed
3. Game closed
    - In following two condition, that all of the submission in the game from a relayer are ignored. (3-1)
      - if someone in the game did not keep submit the blocks in following rounds in the game
      - if the block of someone in the game is verified fail by hash
    - Because some submission are ignored, if all blocks in each round become the only or the same, this block is confirmed, and game is closed. (3-2)

Here is some visualized example with two relayer `Evil` and `Honest`, `Evil` is a bad guy some times relay incorrect header, `Honest` is always relay a correct header.

This is a plot to show Ethereum, G is genesis block or a block already confirmed on Darwinia chain before the game start.
```
         G==========================================>
```

Here is the first submit, `Evil` relay a block with lie(`L`), and `Honest` find out this and relay a truth block with the same ehereum block height,  
so the game starts.  In the meanwhile, the chain can not determine which block is truth or lied, all the information on chain is that there are two different blocks with same block height, at least one of the block with incorrect information.
```
         G======================================1===>
Evil                                            L
Honest                                          H
```
Based on `sample function`, the `Evil` and `Honest` should submit the header on the `2` position (adopted rule 2-2-1.).
```
         G==================2===================1===>
Evil                                            L
Honest                                          H
```
#### From here, the game will become 3 kinds of scenarios, 
  - `Evil` has no response on position 2,
  - `Evil` submit a true block on position 2 honestly.
  - `Evil` submit a block still with lie on position 2.

##### In the first scenario (`Evil` has no response on position 2),
the `Honest` will submit a header on position 2
```
         G==================2===================1===>
Evil                                            L
Honest                      H                   H
```
And waiting the challenge time over, the lie block from `Evil` will be removed (adopted rule 3-1), 
and the only block in each round will be confirmed (denote with 'C') (adopted rule 3-2).
```
         G==================2===================1===>
Evil                                            -
Honest                      C                   C
```

##### In the second scenario (`Evil` submit a block on 2 honestly),
the `Honest` will submit a header on 2
```
         G==================2===================1===>
Evil                        H                   L
Honest                      H                   H
```

And waiting the challenge time over, 
the blocks (the same) in submit round 2 are all confirmed. (adopted rule 2-1)
And `Evil` and `Honest` are still in the game and base on `sample_function`, 
that should submit header on position 3(adopted rule 2-2-1.).
```
         G==================2=========3=========1===>
Evil                        C                   L
Honest                      C                   H
```

##### In the third scenario (`Evil` submit a block still with lie on 2),
the `Honest` will submit a correct header on 2.
```
         G==================2===================1===>
Evil                        L                   L
Honest                      H                   H
```
And there is nothing confirmed without different opinions, 
so base on the `sample_function` the position 3 should be submit by `Evil` and `Honest`.
```
         G=======3==========2===================1===>
Evil                        L                   L
Honest                      H                   H
```
`Evil` and `Honest` can start to submit the block on position 3 when they have different opinion on position 2, 
but the challenge time of submit round 3 will be star counting after run out the challenge time of submit round 2.

#### Pseudo code of relayers-only mode
Here is the [pseudo code](./pseudo/relayers-only/chain.md) of chain, help you to comprehensive this model with multiple relayers in one game.
> the rpc on chain allow anyone to submit headers to challenge blocks still in challenge time, or submit the header according to the sampling function.  The offchain worker keep updating the next sampling block.

Here is the [pseudo code](./pseudo/relayers-only/initial-relayer.md) for the client as the initial relayer
> the client first submit the initial header, and than keep watch the `next_sampling_block`, and submit header of `next_sampling_block`.
>
> submit the initial header   
> while `next_sampling_block`  
> &emsp;submit `next_sampling_block`

Here is the [pseudo code](./pseudo/relayers-only/validating-relayer.md) for the client validating submitting block on chain
> the client first findout a uncorrect initial header, and than keep watch the `next_sampling_block`, and submit header of `next_sampling_block`.
>
> while submit headers  
> &emsp;if verify fail   
> &emsp;&emsp;submit correct block  
>
> while next sample block submit changed  
> &emsp;if the block not correct  
> &emsp;&emsp;submit new correct block  

#### Conclusion of relayers-only mode
- In the first scenario, the game is closed.  
- In the second and third scenario, the game is still going and will be convergence some where between `G` and `1`.

Once the `Evil` goes into contradictory. All of the bond from `Evil` will be slashed, and the slash to reward can be distributed with different functions.  
In the model, no mater there are how many malicious relayers, one honest relayer always response correct information will win the game.  
For a honest relayer, the bond entry barrier is `log2(first submit block - blocks_from_last_comfirm) * bond` and the max game round is `first submit block - blocks_from_last_comfirm`.

### relayer-challenger mode
In relayer-challenger mode, when someone is not accepted the block submitted by other relayer, he just put a challenge on chain to express his opinion.
And currently, we are not consider the challenger do evil scenario.
However, there is still a bond for the challenger to challenge.
1. Any relayer can relay a ethereum header on the darwinia chain
2. Any challenger can challenge the relayer and open the game
    - challenger needs to bond some value for each challenge before the half challenge time over
    - relayer needs to submit the next specified block by `sample function`
3. Game closed
    1. The challenger stops challenging, the relayer wins and challenger will be slashed
    2. The relayer can not provided the next sampling block block pass the validation before the next half challenge time over

Here is some visualized example with relayer `Evil` and challenger `Challenger`, `Evil` is a bad guy some times relay incorrect header, `Challenger` is honest challenger challenge with the correct opinion.

This is a plot to show Ethereum, G is genesis block or a block already confirmed on Darwinia chain before the game start.
```
             G==========================================>
```

Here is the first submit, `Evil` relay a block with lie(`L`), and `Challenger` find out this block is not correct and challenge it with `0`, so the game starts.  
In the meanwhile, the chain still can not determine which block is truth or lied, all the information on chain is that there is a block with dispute.
```
             G======================================1===>
Evil                                                L
Challenger                                          0
```

Based on `sample function`, the `Evil` should submit the block on position 2.
```
             G=================2====================1===>
Evil                                                L
Challenger                                          0
```
#### From here, the game will become 3 kinds of scenarios, 
  - `Evil` has no response on position 2,
  - `Evil` submit a block on position 2 honestly.
  - `Evil` submit a block still with lie on position 2.

##### In the first scenario (`Evil` has no response on position 2),
If `Evil` is not response before the half challenge time over, 
the `Challenger` will win the game and the bond of `Evil` in position 1 will become the be slashed and become reward for `Challenger`.

##### In the second scenario (`Evil` submit a block on position 2 honestly),
If `Evil` submit a correct block in position 2, the challenger will challenge with `0` on position 1 and '1' on position 2.
```
             G=================2====================1===>
Evil                           H                    L
Challenger                     1                    0
```
Such that, based on `sample function`, the next sampling block will be between the position 1 and position 2.
```
             G=================2==========3=========1===>
Evil                           H                    L
Challenger                     1                    0
```
##### In the third scenario (`Evil` submit a block still with lie on 2),
If `Evil` submit a correct block in position 2, the challenger will challenge with `0` on position 1 and '0' on position 2.
```
             G=================2====================1===>
Evil                           L                    L
Challenger                     0                    0
```
Such that, based on `sample function`, the next sampling block will be between the genesis and position 2.
```
             G=======3=========2====================1===>
Evil                           L                    L
Challenger                     0                    0
```

#### Pseudo code of relayer-challenger mode
Here is the [pseudo code](./pseudo/relayer-challenger/chain.md), help you to comprehensive this model with one relayer and one challenger,
Once challenger determine a block pending on chain is correct or not, he will not change his idea.
> The rpc on chain allow relayer to submit headers, and any one to challenge blocks still in challenge time.  
> The offchain worker keep updating the next sampling tartget.

Here is the [pseudo code](./pseudo/relayer-challenger/relayer.md) for the relayer, this code is the same with the initial relayer in `relayers-only` model
> The client first submit the initial header, and than keep watch the `next_sampling_block`, and submit header of `next_sampling_block`.
>
> submit the initial header   
> while `next_sampling_block`  
> &emsp;submit `next_sampling_block`  

Here is the [pseudo code](./pseudo/relayer-challenger/challenger.md) for challenger
> The client first findout a uncorrect initial header and submit a challenge info , and than keep watch the `next_sampling_block`, and keep submit the challenge info base on the relayer's new submit.
>
> while submit headers  
> &emsp;if verify fail  
> &emsp;&emsp;challenge  
> while next sample block submit changed  
> &emsp;verify the block  
> &emsp;submit new challenge  

#### Conclusion of relayer-challenger mode
- In the first scenario, the game is closed.  
- In the second and third scenario, the game is still going and will be convergence some where between `G` and `1`.
  - Following are the assumption, that challenger will beat the evil relayer
    - The fake blocks are not easy to pass validation blocks when near by
    - If challenger is not collusion with the evil relayer.

Once the `Evil` goes into contradictory. All of the bond from `Evil` will be slashed, and the game is closed.  
Please note there is no correct block on position 1 after the game closed, so there may be multiple parallel relayer-challenger games on chain to keep the bridge works.  
For a honest challenger or relayer, the bond entry barrier is `log2(first submit block - blocks_from_last_comfirm) * bond` and the max game round is `log2(first submit block - blocks_from_last_comfirm)`.  
In this model, there is an assumption that the challenger will be honest to keep the bridge secure, so it is required some legal enforcement or high value staking for challenger, 
such that it is not truly decentralized for this model.

### relayer-challengers mode
In relayer-challengers mode, when someone is not accepted the block submitted by other relayer, he just put a challenge on chain to express his opinion.
With multiple challengers, the challengers can take over the challenge jobs, and relay is obliged to all the challeenge from relayers.

1. Any relayer can relay a ethereum header on the darwinia chain
2. Any challenger can challenge the relayer with the challenge info non-exsist
    - challenger needs to bond some value for each challenge before the half challenge time over
    - relayer needs to submit the sampling headers based on the challenge and the `sample function`
3. Game closed
    1. The challengers stop challenging, the relayer wins and all challengers will be slashed
    2. The relayer can not provided the next sampling block block pass the validation before the next half challenge time over

Here is some visualized example with relayer `Evil` and challengers `Challenger1`, `Challenger2`,  `Challenger3`, `Evil` is a bad guy some times relay incorrect header, challengers may not be a honest.

This is a plot to show Ethereum, G is genesis block or a block already confirmed on Darwinia chain before the game start.
```
             G==========================================>
```

Here is the first submit, `Evil` submit a block `B`, and `Challenger1` find out this block is not correct and challenge it with `0`, so the game starts.  
In the meanwhile, the chain still can not determine which block is truth or lied, all the information on chain is that there is a block with dispute.
```
             G======================================1===>
Evil                                                B
Challenger1                                         0
```
Here in `Challenger1` submit a challenge `"0"`, that the length of challenge is 1, and the game is opened.

Based on `sample function`, the `Evil` should submit the block on position 2.
```
             G=================2====================1===>
Evil                                                B
Challenger1                                         0
```

#### From here, the game will become 2 kinds of scenarios, 
  - `Evil` has no response on position 2,
  - `Evil` submit a block on position 2.

##### In the first scenario (`Evil` has no response on position 2),
If `Evil` dose not submit, `Evil` will be slashed, than go to reward stage and reward the correct challenger.  

##### In the second scenario (`Evil` submit block on position 2),
When `Evil` submit a header on position 2, there are also two kind of scenarios.
  - challeneger comfirm with the submission on position2
  - challeneger deny with the submission on position2

The `relayer-challengers` allows multiple challengers with different opions. 
For example, `Challenger2` and `Challenger3` have different opinions on the block at position 2.

`Challenger2` does not comfirm the block on position2 and submit a new challenge `"00"`, that the length of challenge is two.
And based on `Challenger2 `'s 'challenge, the relayer should submit the block on position 3a.
```
             G======3a==========2====================1===>
Evil                            B                    B
Challenger1                                          0
Challenger2                     0                    0
```


`Challenger3` comfirms the block on position2 and submit a new challenge `"01"`, that the length of challenge is two.
And based on `Challenger3 `'s 'challenge, the relayer should submit the block on position 3b.
```
             G======3a==========2=========3b=========1===>
Evil                            B                    B
Challenger1                                          0
Challenger2                     0                    0
Challenger3                     1                    0
```

If `Evil` submit a block on positin 2, there can be challenge with "00" and "01", and the maximum next sampling blocks can be two.
so the max challenge and the maximum next sampling blocks become a tree and follow the following requation.

- `challenge for n round = 2 ^ (submit_round - 1)`
- `total challenge = 2 ^ submit_round - 1`
- `samples for n round = 2 ^ (submit_round - 2)`
- `total samples for n round = 2 ^ (submit_round - 1)`

When there is no new challenge and all the blocks are over the challenge waiting time, all the bond from challengers will be slashed.
If the relayer submit a block can not be validate or contradictory with other submissions, 
the relayer will be slashed, and than go to reward stage and reward the correct challenger. 


#### Reward Stage
The game will be closed when reaching following conditions
- relayer wins
  - There is no new challenger and all challenging time are over
- relayer fail
  - The block can not be varified 
  - The relayer has not response over the challenge waitting time

##### In the first scenario (relayer wins),
All of the challengers will be slash, and reward the relayer.

##### In the second scenario (relayer fail),
The relayer will be slash, all the bond of challenger will be returned, 
and the leaf of challenge make the relayer fail should be rewarded,  also the roots and parents derived from the wining leaf should be rewarded.
These reward are baseed on the slash value of relayer on each submit round.
However there may be still some chllengers we can not check their behavior, so the value they bond will be returned without rewards.

For example, the block height of position 1 is `G+4`, and the block on position `3b` can not be varified with `1`
Following chart show the challenge from three challengers.

```
             G  3a 2  3b 1==>
Evil               B  B  B
Challenger1              0
Challenger2        1     0
Challenger3        0     0
```

Following are the actions for the relayer and challengers
- `Evil` is slashed
- All bond of challengers are returned
- `Challenger1` is rewarded from the relayer's bond of position 1
- `Challenger2` is rewarded from the relayer's bond of position 2
- There is no reward for `Challenger3`.

Another example, the block height of position 1 is `G+4`, and all the blocks are valid.
Following chart show the challenge from three challengers.


```
             G  3a 2  3b 1==>
Honest          B  B  B  B
Challenger1              0
Challenger2        1     0
Challenger3        0     0
```

Following are the actions for the relayer and challengers
- All bond from `Honest` relayer is returned
- All challengers are be slashed as reward to relayer

The slash and reward can be related as following:
- The value slashed from `Challenger1` is as reward for the submission of block 2
- The value slashed from `Challenger2` is as reward for the submission of block 3b
- The value slashed from `Challenger3` is as reward for the submission of block 3a

#### Pseudo code of relayer-challengers mode
Here is the [pseudo code](./pseudo/relayer-challengers/chain.md), help you to comprehensive this model with one relayer and one challenger,
Once challenger determine a block pending on chain is correct or not, he will not change his idea.
> The rpc on chain allow relayer to submit headers, and any one to challenge blocks still in challenge time.  
> The offchain worker keep updating the next sampling blocks.

Here is the [pseudo code](./pseudo/relayer-challengers/relayer.md) for the relayer, this code is the same with the initial relayer in `relayers-only` model
> The client first submit the initial header, and than keep watch the list of `next_sampling_blocks`, and submit each header listed in `next_sampling_blocks`.
>
> submit the initial header  
> while `next_sampling_blocks`  
> &emsp;submit each header listed in `next_sampling_blocks`  

Here is the [pseudo code](./pseudo/relayer-challengers/challenger.md) for challenger
> The client first findout an uncorrect initial header and submit a challenge info , and than keep watch the `next_sampling_blocks`, and keep submit the challenge info base on the relayer's new submit.
>
> while submit headers  
> &emsp;if verify fail  
> &emsp;&emsp;challenge  
> while submit headers  
> &emsp;if next sample block submit  
> &emsp;&emsp;verify the block  
> &emsp;&emsp;submit new challenge  


#### Conclusion of relayer-challengers mode
In this model, we are not determine each block in different round is correct or not.  
We just make sure we have a solution which can always to challenge a evil relayer and let him to provide more information on chain.  
Once the relayer contradictory itself the relay is slashed and the game is close.  
On the other hand, the honest relayer can get the corresponding rewards for each block from the corresponding slash of challenge.  
For a honest relayer, the bond entry barrier is `blocks_from_last_comfirm * bond` and the max game round is `first submit block - blocks_from_last_comfirm`.  
For a honest challenger, the bond entry barrier is `log2(first submit block - blocks_from_last_comfirm) * bond` and the max game round is `log2(first submit block - blocks_from_last_comfirm)`.  
The challenging time of block may be extended with `graceful period` for relayer only.
The `graceful period` will be calculate by `graceful_function` when implementing.

### relayers-take-over mode
The `relayer-take-over` mode is similar to the `relayer-challengers` mode, and the challenger need to provide header to express the different opinions.
In this mode the challengers should submit header to prevent the evil challengers to malresponse easy and DoS the system.
However, there is still no the rule `Once in participate all` for `Estoppel`, so there is some rare case wihtout comfirm block at all.

Here in, the plots is converted from the second scenario (`Evil` submit block on position 2) in `relayer-challengers` mode, 
that relayers submit the blocks `a` to `e`, and the Evil decides to quit the game without response on `3a` and `3b`.

```
             G======3a==========2=========3b=========1===>
Evil                            b                    a      Slash
Challenger1                                          c      Return
Challenger2                     d                    c      Return  (take over from Challenger1)
Challenger3                     b                    e      Return  
```
The game is closed and `c` is **not** confirmed, because of `e`.

The results are 3 status, following 2 cases help you to know more about this.

**Case 1**
```
             G======3a==========2=========3b=========1===>
Evil                            b                    a      
Challenger1                                          c     
Challenger2                     d                    c    
Challenger3                                          e   
```
Only Challenger2 beat the Evil, so we can deem the result from Challenger 2 is correct.
So Challenger1 and Challenger2 got the reward, and the `c` is confirmed as following plot.
```
             G======3a==========2=========3b=========1===>
Evil                            -                    -      Slash
Challenger1                                          C      Reward
Challenger2                     C                    C      Reward
Challenger3                                          -      Slash
```

**Case 2**
```
             G======3a==========2=========3b=========1===>
Evil                            b                    a      Slash
Challenger1                                          c      Return
Challenger2                     d                    c      Return   (take over from Challenger1)
Challenger3                                          e      Return
Challenger4                     b                    e      Return   (take over from Challenger3)
```
Challenger2 and Challenger4 beat the Evil.
Without `Once in participate all` and `Estoppel`, the possible blocks in position 1 are `C`, `E`.
`A` is eliminated, because the initial relayer having responsibility to keep relay the sampling blocks.
There is no rule to eliminate blocks `C` or `E`, so there is no confirm block.

And let us using Honest(`H`) and Lie(`L`) to show the four possible sub-cases.

**Case 2-1**
```
             G======3a==========2=========3b=========1===>
Evil                            H                    L      Slash
Challenger1                                          H      Return
Challenger2                     L                    H      Return   (take over from Challenger1)
Challenger3                                          L      Return
Challenger4                     H                    L      Return   (take over from Challenger3)
```
**Case 2-2**
```
             G======3a==========2=========3b=========1===>
Evil                            H                    L      Slash
Challenger1                                          L      Return
Challenger2                     L                    L      Return   (take over from Challenger1)
Challenger3                                          H      Return
Challenger4                     H                    H      Return   (take over from Challenger3)
```
**Case 2-3**
```
             G======3a==========2=========3b=========1===>
Evil                            L                    L      Slash
Challenger1                                          L      Return
Challenger2                     H                    L      Return   (take over from Challenger1)
Challenger3                                          H      Return
Challenger4                     L                    H      Return   (take over from Challenger3)
```
**Case 2-4**
```
             G======3a==========2=========3b=========1===>
Evil                            L                    L      Slash
Challenger1                                          H      Return
Challenger2                     H                    H      Return   (take over from Challenger1)
Challenger3                                          L      Return
Challenger4                     L                    L      Return   (take over from Challenger3)
```

Therefor, if the game rarely stop as the status show in the **Case 2**, we just slash Evil and return the bond for challengers as following plot.
```
             G======3a==========2=========3b=========1===>
Evil                            -                    -      Slash
Challenger1                                          -      Return
Challenger2                     -                    -      Return   (take over from Challenger1)
Challenger3                                          -      Return
Challenger4                     -                    -      Return   (take over from Challenger3)
```
This is the worst case for this mode, nothing is confirmed.
However, in optimistic game, there is always a good guy in each round.  
Such that the good guy will return in position `3a` or `3b` to beat Challenger2 or Challenger4.

## Sample Function
Sample function is an equation to provide the block height numbers, that relayer should submit the blocks at that block height.
Sample function is the key part to prevent the attacker, and also determine the total consuming time in relayer game.
And it is reasonable for using different sample equation for different target chain with different consensus algorithm.
Following listed are the design philosophy.  
- Transparent and with ambiguous part
  - The sample equation should be clear and transparent for people, and there will be also some ambiguous part provided by random number, such that the attacker need much affair to making fake headers.  
- Sampling the tail at first
  - By nature, the **PoW** consensus mechanism, the branch will occur and not greater than a reasonable length, for example 6.  To accelerate the process of relayer verification game, the sample function will label the `N-6` to `N-1` blocks at the second round, such that the nature branch point can be find out as soon as possible.
- Sampling the blocks near by the confirmed blocks on chain
  - It is easy to find out the counterfeit block which is near by a confirmed block

## Stage Two
In stage two of the relayer verification game, the nature branch will be solved.
When a relayer with dispute on chain but all blocks is correct, the challenger or second relayer can ask to open the stage two of the game.
Here is status when the stage two opening in `relayers-only` mode.

```
               G==============================nnnn1=====>
InitialRelayer                                Caaaa
Relayer2                                      Cbbbb
```
**C**: is the latested comfirm block

Only the longest validated chain will be accepted in blockchain network.
Relayer 2 start to provide the chain as long as posible after the point of first submission to open the stage two.

```
               G==============================nnnn1=====>
InitialRelayer                                Caaaa
Relayer2                                      Cbbbbbbbb
```

Then, the initial relayer should provide a longer chain to prove that he is the longest validated chain in the challenge time as following.
```
               G==============================nnnn1=====>
InitialRelayer                                Caaaaaaaaa
Relayer2                                      Cbbbbbbbb
```

If Relayer2 can still have keep challenge by providing more headers, and so on.

The stage two of game should be rare, because relayer should submit a block already finialized, bute we still design the stage two to solve the branch issue just in case.

## General Parameters
- `title ` (optional)
  - The title for this scenario will print on the console
- `F` (optional)
  - The block producing factor for Darwinia / Ethereum
  - For example: 2.0, that means that Darwinia produce 2 blocks and Ethereum produce 1 block.

## Specify Functions Type
- `challenge_function`
  - Once a relayer submit a header and challenge the time in blocks after the calculated value from challenge function, 
    Darwinia network will deem this header is valided and become a best header.  
  - Current support: integer number, linear   
  - For example: '10', that means a submit block will be deem to relayed and finalized after 10 Darwinia blocks.
  - For example: `challenge_linear`, that means a submit block will wait according the linear function, and the parameters of function need to provide.

- `sample_function`
  - Once there is dispute on any header, the relayer should submit the next Ethereum block sampling as calculated.  
  - Current support: half
  - For example: 'half', that means the next sampling block will be `(submited_ethereum_block_height - relayed_ethereum_block_height) / 2`

- `bond_function`
  - The bond function will increase the bond to improve speed of the finality, and the cost of keeping lie will be enormous.  
  - Current support: float number, linear   
  - For example: '10.0', that means the bond of each submit is always 10.0.

- `reward_function`
  - The reward from relayers including the treasury and the slash from the lie relayers (attackers)
  - It may be reasonable when that the slash part of reward should be the same as bond functions, 
    but both the first round submit and the second round submit are to help to beat the attackers in the first round, 
    so the slash from the first round sumit may split some portion for the honest relayers in the second round.
  - And also it may be that the treasury part is only for the last submit rounds, if the slash never split to the next round
    - the treasury part is from the fee of redeem action, but it will be a debt without limitation in simulation

## Initialize Status of Darwinia and Ethereum
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
    - if the length of chose are shorter than other relayers, it will be deem to no response.  

We assume there always is a good guy to relay the correct headers, and the guy will name `Darwinia`, 
and this relayer will be automatic add into the scenario when load from configure file, 
so please avoid to use this name for the relayer.

## Challengers' Chose
The following parameters are used for challenger
- `[[challengers]]` 
  - `name` (optional)
    - if name is not provided, the relayer will be name with serial numbers
  - `choice`
    - challenger may be response as following
      - `1`(agree with relayer, this means relayer is honest at this round)
      - `0`(disagree with relayer, this means relayer lies at this round)

## Parameters of Equation
The three function can use different equations, base on the function setting, following parameters of function should be filled.
- `[challenge_linear]`
  - the linear equation for challenge function
  - `challengeing block = int(min(Wd * D, Md) + min(We * E, Me)) + C`
  - suffix `d`: block difference between last block number relayed on Darwinia, suffix `e`: block difference between last related block number of Ethereum
  - D is the distance in Darwinia chain
  - E is the distance in Ethereum chain
  - W is the weight for that portion
  - M is the maximum value for that portion

- `[bond_linear]`
  - the linear equation for bond function
  - `bond = min(W * E, M) + C`
  - W is the weight 
  - M is the maximum value for the variable part

- `[reward_split]`
  - Split the slash for reward the honest relayers in two rounds
  - slash value of submit round will take P as reward in current round, and leave (1-P) for the next round
  - P is the portion 
  - the slash value for this submit round is slash value * P
  - the slash value for this next submit round is slash value * (1 - P)

- `[reward_treasury_last]`
  - The slash for reward the honest relayers in the same round
  - The treasury will reward the relayers in the last submit round, because there is no attacker(lie relayers) in the last round.
  - C is the constant of the reward from treasury

## Build & Run
This executable is written in Rust, it can be easily to build and run with cargo command.  
```
cargo build --release
```
then the binary will be placed in ./target/release, you can run this command with scenario file as following command.  
```
./target/release/refit scenario/basic.yml
```
Also, you can put `-v` option to see all status in each round of submit.
```
./target/release/refit -v scenario/multi-challengers2.yml
```
following picture is the example what you will see with verbose flag
![snapshot](https://raw.githubusercontent.com/yanganto/relayer-game/master/demo-vebose.png)

Besides, you can patch some equation parameters with option `p`, for examples.
```
./target/release/refit -p challenge_linear.C=9 challenge_linear.Wd=10.0 -- scenario/basic.yml
```
Currently, all parameters in `challenge_linear` and `bond_linear`, and also the values of `challenge_function` and `bond_function` can be patched.  
The challenge times(in blocks), and the bonds for each round will show as plot help you to modify the equation.  
![snapshot](https://raw.githubusercontent.com/yanganto/relayer-game/master/demo.png)

After running this tool, the reward and slash from each relayer will show as following picture.
![snapshot](https://raw.githubusercontent.com/yanganto/relayer-game/master/demo2.png)

If you want to use this tool without plot with a smaller binary, please use `--no-default-features` option when building.
```
cargo build --release --no-default-features
```

## Develop and Document
This project has document, you can use this command to show the document on browser.
`cargo doc --no-deps --open`
If you want to add more equation for different function, you can take a look the trait in [bond](./src/bond/mod.rs), [challenge](./src/challenge/mod.rs), [sample](./src/sample/mod.rs).
The `Equation` trait and `ConfigValidate` will guild you to add you customized equation. 
