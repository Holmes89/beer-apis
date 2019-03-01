package internal

type Beer struct {
	ID        int64    `json:"id" db:"id"`
	BreweryID int64    `json:"brewery_id" db:"brewery_id"`
	Name      string `json:"name" db:"name"`
}

type Brewery struct {
	ID      int64    `json:"id"  db:"id"`
	Name    string `json:"name"  db:"name"`
	Address string `json:"address"  db:"address"`
	City    string `json:"city"  db:"city"`
	State   string `json:"state"  db:"state"`
	Code    string `json:"zip_code"  db:"zip_code"`
}
