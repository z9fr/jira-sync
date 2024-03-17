# Jira Sync

The idea is to implement a way to sync my jira task time log to clockify. should be able to easily add time to jira and sync to clockify easily

## Steps 

- dump the task list (where i have't logged time) as a csv
- update the time log on the csv
- run sync which will automatically sync in both ends


---

### TODO:

- implement a way to ignore the slots which already has been used for clockify


> Note: the api's im using are not bulk endpoints since the current operations im using does not support bulk op. 
so this can rate limit you if you try to update a big number at once. 
