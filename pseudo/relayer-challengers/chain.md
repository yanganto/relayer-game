```python
# game status stored on chain
submit_headers = []
challenge_list = []
next_sampling_blocks = []
challege_time_in_blocks = 100 # wait 100 blocks for challenge time, here is a simplify constant waiting time
last_comfirm_block_height = genesis_block_height  # or the block height already comfirm before the game

def header_submit_by_relayer(header):
  if next_sampling_blocks and header.block_height not in  next_sampling_blocks:
    return Err("Submission is not in next sampling blocks")
  if header.block_height in  next_sampling_blocks:
    next_sampling_blocks.pop(header.block_height)

  relayer = ensure_signed()  # this function will return the identity of the relayer
     
  if validate(header, submit_headers):  # validate header and check if contradictory or not
    heaser.relayer = relayer
    header.challenge_block_height = current_block_height + challege_time_in_blocks 
    submit_headers.append(header)
  else:
    slash_relayer_and_reward_challenger()
    close_game()


def challenge(challenge_info):
  if challenger != ensure_signed():   # the identity of challenger are different
    return Err("There is a challenger")
  elif len(challenge_info) > len(submit_headers): 
    return Err("challenge info is not correct")
  elif challenge_info in challenge_list:
    return Err("The challenge already exsist")

  for c in challenge_info:
    if c not in ("0", "1"):
      return Err("challenge info is not correct")

  challenge_list.append(challenge_info)
  if len(challenge_info) > 1
    c = calculate_last_comfirm_blocks(challenge_info)  # the last comfirm block are different based on different challenge after the secondary round
  else:
    c = last_submit_block_height
  next_sampling_blocks.append(in_the_middle_of(c, submit_headers[-2].block_height))


def offchain_worker():
  """ the proccess will called for each block based on substrate """
  has_challenge_waiting_blocks = False
  for b in submit_headers:
    if b.challenge_block_height >= current_block_height: 
      has_challenge_waiting_blocks = True

  if not has_challenge_waiting_blocks:
    slash_challenger_and_reward_relayer()
    close_game()

def close_game()
  """ reset status on chain """
  submit_headers = []
  next_sampling_blocks = []
```
