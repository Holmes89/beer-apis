(ns clj-beer-api.repo
  (:require [clojure.java.jdbc :as sql]))

(def db-spec {:classname "org.sqlite.JDBC"
              :subprotocol "sqlite"
              :subname "resources/beers.db"})

(defn find-all-breweries
  [offset size]
  (sql/query db-spec ["SELECT * From breweries LIMIT ?,?" offset size]))

(defn find-brewery-by-id
  [id]
  (sql/query db-spec ["SELECT * FROM breweries where id = ?" id]))

(defn find-beer-by-id
  [id]
  (sql/query db-spec ["SELECT * FROM beers where id = ?" id]))

(defn find-all-beers
  [offset size]
  (sql/query db-spec ["SELECT * FROM beers LIMIT ?,?" offset size]))

(defn find-all-brewery-beers
  [id]
  (sql/query db-spec ["SELECT * FROM beers WHERE brewery_id = ?" id]))
