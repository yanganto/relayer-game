```python
# game status stored on chain
submit_headers = OrderedDefaultdict(list)  # implement __missing__ for OrderedDict, than you can get this type
next_sampling_block_height = None
last_comfirm_block_height = 0   # the block height of gensis
challege_time_in_blocks = 100   # wait 100 blocks for challenge time, here is a simplify constant waiting time

def header_submit_by_relayer(header):
  if header.block_height in submit_headers.keys() and submit_headers[header.block_height][0].challenge_block_height < current_block_height: 
    return Err("Block is comfirmed")
  elif submit_headers.keys() and header.block_height != next_sampling_block_height:
    return Err("Submission is not a next_sampling block")

  relayer = ensure_signed()  # this function will return the identity of the relayer

  if header not in submit_headers[header.block_height]:  # implement __equal__ to check only the block info but not relayers and challenge_block_height
    if validate(relayer, header, submit_headers):  # validate header and check not contradictory
      if submit_headers.keys():
        last_challenge_wait_block_height = submit_headers[submit_headers.keys()[-1]][0].challenge_block_height 
      else:
        last_challenge_wait_block_height = current_block_height
      header.challenge_block_height = last_challenge_wait_block_height + challege_time_in_blocks 
      header.relayers = [relayer]
      submit_headers[header.block_height].append(header)

      if len(submit_headers[header.block_height]) == 2:  # dispute occure, more than one submit the same block height
       next_sampling_block_height = in_the_middle_of(last_comfirm_block_height, header.block_height)
  else:
    submit_headers[header.block_height].relayers.append(relayer)

def offchain_worker():
  """ the proccess will called for each block based on substrate """
  last_submit_block_height = submit_headers.keys()[-1]

  if len(submit_headers[last_submit_block_height]) == 1 and 
      submit_headers[last_submit_block_height][0].challenge_block_height < current_block_height:

    honest_relayers = submit_headers[last_submit_block_height].relayers

    slash_and_reward()

  if no_uncomfirm_blocks():
    close_game()

def close_game()
  """update the last comfirm block and reset the status on chain"""
  last_comfirm_block_height = submit_headers[submit_headers.keys()[0]].block_height
  submit_headers = OrderDefaultdict(list)
  next_sampling_block_height = None


def validate(relayer, header, submit_headers):
  """validate the block"""
  # 1. basic block information check, for example, validate mix_hash, difficulty, in ethereum
  # 2. check the block in not controversy with the blocks in `submit_headers` and submited by `relayer`
```
