```python
last_submit_block_height = None

while chain.submit_headers:  # check there is a game 
  for _, header in chain.submit_headers.items():
    if !validation(header):  # find the block is not correct
      header = get_header_by_block_height(chain.next_sampling_block_height)
      chain.header_submit_by_relayer(header)  
      last_submit_block_height = header.block_height
      break  # now the game start, and then just watching the next_sampling block

while chain.next_sampling_block_height:
  if chain.next_sampling_block_height != last_submit_block_height:  # This means the next_sampling has updated
    header = get_header_by_block_height(chain.next_sampling_block_height)
    last_submit_block_height = header.block_height
    chain.header_submit_by_relayer(header)  
```
