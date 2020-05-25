```python
class ChallengeInfo(UserDict):
  def agree_with(self, header):
    return self[header.block_height]

last_submit_block_height = None
challenge_info = ChallengeInfo()

while chain.submit_headers:  # check there is a game 
  for _, header in chain.submit_headers:
    if !validation(header):  # find the block is not correct
      challenge_info[header.block_height] = false
      chain.challenge(challenge_info)  
      last_submit_block_height = header.block_height
      break  # now the game start, and then just watching the next sampling block

while chain.next_sampling_block_height:
  if chain.next_sampling_block_height != last_submit_block_height:  # This means the next sampling has updated
    next_sampling_block_is_valid = check_the_block_on_chain_correct_or_not(chain.next_sampling_block_height, chain.submit_headers)
    if next_sampling_block_is_valid is not None:
      challenge_info[chain.next_sampling_block_height] = next_sampling_block_is_valid
      last_submit_block_height = header.block_height
      chain.challenge(challenge_info)  

def check_the_block_on_chain_correct_or_not(block_height, headers):
  for h in headers:
    if h.block_height == block_height
      return validate(h)
  else:
    return None  # relayer still not submit yet, keep waiting
```

