```python
chain.header_submit_by_relayer(first_header)  # submit the first block

while chain.submit_headers:  # check game is closed or not
  if chain.next_sampling_blocks:  # This means the next_sampling has updated
    for b in chain.next_sampling_blocks:
      header = get_header_by_block_height(b)
      chain.header_submit_by_relayer(header)  
```
