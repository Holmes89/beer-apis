package internal

import (
	"context"
	"crawshaw.io/sqlite"
	"errors"
	"github.com/sirupsen/logrus"
)

type Database interface {
	FindBreweryByID(ctx context.Context, id string) (*Brewery, error)
	FindBeerByID(ctx context.Context,id string) (*Beer, error)
	FindAllBreweries(ctx context.Context, page, size int) ([]*Brewery, error)
	FindAllBeers(ctx context.Context, page, size int) ([]*Beer, error)
	FindAllBreweryBeers(ctx context.Context, id string) ([]*Beer, error)
}

type sqliteDB struct {
	pool *sqlite.Pool
}

func NewBeerDB() (Database, error) {
	dbpool, err := sqlite.Open("file:beers.db", 0, 10)
	if err != nil {
		logrus.WithField("err", err.Error()).Error("unable to create db connection pool")
		return nil, err
	}
	return &sqliteDB{
		pool: dbpool,
	}, nil
}

func (r *sqliteDB) FindBreweryByID(ctx context.Context, id string) (*Brewery, error) {
	conn := r.pool.Get(ctx.Done())
	if conn == nil {
		logrus.Error("no connections available in pool")
		return nil, errors.New("no connection available")
	}
	defer r.pool.Put(conn)

	stmt := conn.Prep("SELECT * FROM breweries WHERE id = $id;")
	stmt.SetText("$id", id)

	var brewery *Brewery
	for {
		if hasRow, err := stmt.Step(); err != nil {
			logrus.WithField("err", err.Error()).Error("statement creation failed")
			return nil, err
		} else if !hasRow {
			break
		}
		brewery = r.breweryFromStatement(stmt)
	}

	return brewery, nil
}

func (r *sqliteDB) FindBeerByID(ctx context.Context,id string) (*Beer, error) {
	conn := r.pool.Get(ctx.Done())
	if conn == nil {
		logrus.Error("no connections available in pool")
		return nil, errors.New("no connection available")
	}
	defer r.pool.Put(conn)

	stmt := conn.Prep("SELECT * FROM beers WHERE id = $id;")
	stmt.SetText("$id", id)

	var beer *Beer
	for {
		if hasRow, err := stmt.Step(); err != nil {
			logrus.WithField("err", err.Error()).Error("statement creation failed")
			return nil, err
		} else if !hasRow {
			break
		}
		beer = r.beerFromStatement(stmt)
	}

	return beer, nil
}

func (r *sqliteDB) FindAllBreweries(ctx context.Context,page, size int) ([]*Brewery, error){
	conn := r.pool.Get(ctx.Done())
	if conn == nil {
		logrus.Error("no connections available in pool")
		return nil, errors.New("no connection available")
	}
	defer r.pool.Put(conn)

	offset := page * size;
	stmt := conn.Prep("SELECT * FROM breweries LIMIT $offset,$size;")
	stmt.SetInt64("$offset", int64(offset))
	stmt.SetInt64("$size", int64(size))

	var breweries []*Brewery
	for {
		if hasRow, err := stmt.Step(); err != nil {
			logrus.WithField("err", err.Error()).Error("statement creation failed")
			return nil, err
		} else if !hasRow {
			break
		}

		brewery := r.breweryFromStatement(stmt)
		breweries = append(breweries, brewery)
	}

	return breweries, nil
}

func (r *sqliteDB) FindAllBeers(ctx context.Context,page, size int) ([]*Beer, error) {
	conn := r.pool.Get(ctx.Done())
	if conn == nil {
		logrus.Error("no connections available in pool")
		return nil, errors.New("no connection available")
	}
	defer r.pool.Put(conn)

	offset := page * size;
	stmt := conn.Prep("SELECT * FROM breweries LIMIT $offset,$size;")
	stmt.SetInt64("$offset", int64(offset))
	stmt.SetInt64("$size", int64(size))

	var beers []*Beer
	for {
		if hasRow, err := stmt.Step(); err != nil {
			logrus.WithField("err", err.Error()).Error("statement creation failed")
			return nil, err
		} else if !hasRow {
			break
		}

		beer := r.beerFromStatement(stmt)
		beers = append(beers, beer)
	}

	return beers, nil
}

func (r *sqliteDB) FindAllBreweryBeers(ctx context.Context, id string) ([]*Beer, error) {
	conn := r.pool.Get(ctx.Done())
	if conn == nil {
		logrus.Error("no connections available in pool")
		return nil, errors.New("no connection available")
	}
	defer r.pool.Put(conn)

	stmt := conn.Prep("SELECT * FROM beers where brewery_id = $id")
	stmt.SetText("$id", id)

	var beers []*Beer
	for {
		if hasRow, err := stmt.Step(); err != nil {
			logrus.WithField("err", err.Error()).Error("statement creation failed")
			return nil, err
		} else if !hasRow {
			break
		}

		beer := r.beerFromStatement(stmt)
		beers = append(beers, beer)
	}

	return beers, nil
}

func (r *sqliteDB) breweryFromStatement(stmt *sqlite.Stmt) *Brewery {
	return &Brewery{
		ID: stmt.GetInt64("id"),
		Name: stmt.GetText("name"),
		Address: stmt.GetText("address"),
		City: stmt.GetText("city"),
		State: stmt.GetText("state"),
		Code: stmt.GetText("zip_code"),
	}
}

func (r *sqliteDB) beerFromStatement(stmt *sqlite.Stmt) *Beer {
	return &Beer{
		ID: stmt.GetInt64("id"),
		BreweryID: stmt.GetInt64("brewery_id"),
		Name: stmt.GetText("name"),
	}
}