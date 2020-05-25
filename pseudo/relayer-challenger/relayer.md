```python
last_submit_block_height = first_header.block_height
chain.header_submit_by_relayer(first_header)  # submit the first block

while chain.submit_headers:  # check game is closed or not
  if chain.next_sampling_block_height != last_submit_block_height:  # This means the next_sampling has updated
    header = get_header_by_block_height(chain.next_sampling_block_height)
    last_submit_block_height = header.block_height
    chain.header_submit_by_relayer(header)  
```
