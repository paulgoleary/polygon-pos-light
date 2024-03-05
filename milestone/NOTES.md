
### Inputs to proof (?)

* RLP encoded `data` from milestone side tx
  * fixed size 192? hex? or can it be passed as binary? 
* `sigs` data from milestone side tx
  * can likely be decoded from json and encoded into fixed 65 byte binary
  * Q: how / where to get a/o validate validator set inclusion and stake weight?
* RLP encoded block headers, starting from the block in the milestone that contains the target root hash, through the last block of the milestone. The hash of the last block in the milestone should match what was attested in the milestone message.

### Sketch of milestone proof (?)

* Do RLP decode of milestone message data
* For initial block header, do partial decode and check that:
  * Block num matches target a/o is in range of milestone
  * State root hash matches target hash
* For each subsequent block header, do partial decode and check that:
  * Block num is in range of milestone
  * Parent hash matches hash of previous block
* Additionally, for last block header in milestone, check that hash matches value attested in milestone message data
* Validate all signatures over 'side transaction' hash of message data
  * May require tx hash to be passed in?

### Questions / Issues

* As mentioned above, how do we get and validate the stake weight of the validators that signed in the milestone?
  * Likely needs to be relative to validator epoch of milestone?
