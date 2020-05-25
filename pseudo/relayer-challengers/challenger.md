```python
next_sampling_block = None
challenge_info = ""

while chain.submit_headers:  # check there is a game 
  for _, header in chain.submit_headers:
    if !validation(header):  # find the initial submit block is not correct
      challenge_info += "0"
      chain.challenge(challenge_info)  
      next_sampling_block = calculate_next_sampling_block(header.block_height, chain.)
      break  # now the game start, and then just watching the next sampling block

while chain.submit_headers:
  if next_sampling_block in map(lambda x: x.block_height, submit_headers):  
    header = filter(lambda x: x.block_height == next_sampling_block, submit_headers)[0]:

    if validation(header):  # find out the block is not correct
      challenge_info += "1"
    else:
      challenge_info += "0"

    next_sampling_block = calculate_next_sampling_block(header.block_height, chain.)
    chain.challenge(challenge_info)  
```

