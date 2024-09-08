// When a user sends a PUT or DELETE request, it is first recorded in the Write-Ahead Log (Commit Log) WAL.
// ● Once the WAL confirms the entry, the data needs to be added to the Memtable, which is strictly stored in memory.
// ● When the predefined size of the Memtable is reached, the values are sorted by key, and a new SSTable is created and written to disk.
// ● After that, we check if the conditions for starting a compaction are met, and initiate them if they are. It is important to note that compactions at one level can trigger compactions at the next level.

