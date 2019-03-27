(ns clj-beer-api.service
  (:require [io.pedestal.http :as http]
            [io.pedestal.http.route :as route]
            [io.pedestal.http.body-params :as body-params]
            [io.pedestal.interceptor :as interceptor]
            [cheshire.core :as json]
            [clojure.tools.logging :as log]
            [clj-beer-api.repo :as repo]))

(def content-length-json-body
  (interceptor/interceptor
    {:name ::content-length-json-body
     :leave (fn [context]
              (let [response (:response context)
                    body (:body response)
                    json-response-body (if body (json/generate-string body) "")
                    ;; Content-Length is the size of the response in bytes
                    ;; Let's count the bytes instead of the string, in case there are unicode characters
                    content-length (count (.getBytes ^String json-response-body))
                    headers (:headers response {})]
                (assoc context
                       :response {:status (:status response)
                                  :body json-response-body
                                  :headers (merge headers
                                                  {"Content-Type" "application/json;charset=UTF-8"
                                                   "Content-Length" (str content-length)})})))}))

(def custom-interceptors [content-length-json-body])

(defn breweries
  [request]
  (let [page (get-in request [:query-params :page])]
    {:status 200 :body (repo/find-all-breweries page)}))

(defn brewery
  [request]
  (let [id (get-in request [:path-params :brewery-id])]
    {:status 200 :body (repo/find-brewery-by-id id)}))

(defn brewery-beers
  [request]
  (let [id (get-in request [:path-params :brewery-id])]
    {:status 200 :body (repo/find-all-brewery-beers id)}))

(defn beers
  [request]
  (let [page (get-in request [:query-params :page])]
    {:status 200 :body (repo/find-all-beers page)}))

(defn beer
  [request]
  (let [id (get-in request [:path-params :beer-id])]
    {:status 200 :body (repo/find-beer-by-id id)}))

;; Tabular routes
(def routes
  #{["/brewery-beer/:brewery-id" :get (conj custom-interceptors `brewery-beers)]
    ["/brewery/:brewery-id" :get (conj custom-interceptors `brewery)]
    ["/brewery/" :get (conj custom-interceptors `breweries)]
    ["/beer/" :get (conj custom-interceptors`beers)]
    ["/beer/:beer-id" :get (conj custom-interceptors `beer)]})

;; Consumed by clj-beer-api.server/create-server
;; See http/default-interceptors for additional options you can configure
(def service {:env :prod
              ;; You can bring your own non-default interceptors. Make
              ;; sure you include routing and set it up right for
              ;; dev-mode. If you do, many other keys for configuring
              ;; default interceptors will be ignored.
              ;; ::http/interceptors []
              ::http/routes routes

              ;; Uncomment next line to enable CORS support, add
              ;; string(s) specifying scheme, host and port for
              ;; allowed source(s):
              ;;
              ;; "http://localhost:8080"
              ;;
              ;;::http/allowed-origins ["scheme://host:port"]

              ;; Tune the Secure Headers
              ;; and specifically the Content Security Policy appropriate to your service/application
              ;; For more information, see: https://content-security-policy.com/
              ;;   See also: https://github.com/pedestal/pedestal/issues/499
              ;;::http/secure-headers {:content-security-policy-settings {:object-src "'none'"
              ;;                                                          :script-src "'unsafe-inline' 'unsafe-eval' 'strict-dynamic' https: http:"
              ;;                                                          :frame-ancestors "'none'"}}

              ;; Root for resource interceptor that is available by default.
              ::http/resource-path "/public"

              ;; Either :jetty, :immutant or :tomcat (see comments in project.clj)
              ;;  This can also be your own chain provider/server-fn -- http://pedestal.io/reference/architecture-overview#_chain_provider
              ::http/type :jetty
              ;;::http/host "localhost"
              ::http/port 8080
              ;; Options to pass to the container (Jetty)
              ::http/container-options {:h2c? true
                                        :h2? false
                                        ;:keystore "test/hp/keystore.jks"
                                        ;:key-password "password"
                                        ;:ssl-port 8443
                                        :ssl? false}})

