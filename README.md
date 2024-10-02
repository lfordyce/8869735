# 8869735

## Usage:
```shell
# create movie
 curl -v --header "Content-Type: application/json" -d '{"id": "1", "name": "movie_name", "year": 2024, "was_good": true}' -X POST localhost:3000/movie
# retrieve movie by id
curl -v localhost:3000/movie/2
```