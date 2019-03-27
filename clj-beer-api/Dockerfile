FROM java:8-alpine
MAINTAINER Your Name <you@example.com>

ADD target/clj-beer-api-0.0.1-SNAPSHOT-standalone.jar /clj-beer-api/app.jar

EXPOSE 8080

CMD ["java", "-jar", "/clj-beer-api/app.jar"]
