```python
# game status stored on chain
submit_headers = []
next_sampling_block_height = None
last_comfirm_block_height = 0 # the block height of gensis
challege_time_in_blocks = 100 # wait 100 blocks for challenge time, here is a simplify constant waiting time
challenger = None             # This simplify model only have one relayer and one challenger

def header_submit_by_relayer(header):
  if next_sampling_block_height is not None and header.block_height != next_sampling_block_height:
    return Err("Submission is not next_sampling block")

  relayer = ensure_signed()  # this function will return the identity of the relayer
     
  if validate(header, submit_headers):  # validate header and check if contradictory or not
    heaser.relayer = relayer
    header.challenge_block_height = current_block_height + challege_time_in_blocks 
    submit_headers.append(header)
  else:
    slash_relayer_and_reward_challenger()
    close_game()


def challenge(challenge_info):
  if challenger is None:
    challenger = ensure_signed() 
  elif challenger != ensure_signed():   # the identity of challenger are different
    return Err("There is a challenger")
  elif submit_headers[-1].challenge_block_height < current_block_height: 
    return Err("game is closed")

  for c in challenge_info.keys():
    if c not in map(lambda h: h.block_height, submit_headers):
      return Err("challenge info is not correct")
      
    last_comfirm_block_height = submit_headers[-1].block_height
  next_sampling_block_height = in_the_middle_of(last_comfirm_block_height, submit_headers[-2].block_height)


def offchain_worker():
  """ the proccess will called for each block based on substrate """
  if submit_headers[-1].challenge_block_height < current_block_height:
    slash_challenger_and_reward_relayer()
    close_game()

def close_game()
  """ reset status on chain """
  submit_headers = []
  next_sampling_block_height = None
  challenger = None             
```
