FROM amazoncorretto:8-alpine
ARG port
ARG delay
ENV PORT=$port
ENV DELAY=$delay
WORKDIR /app
ADD target/dummy-sse-services-1.0-SNAPSHOT-shaded.jar /app/dummy-sse-services-1.0-SNAPSHOT-shaded.jar
CMD java -jar /app/dummy-sse-services-1.0-SNAPSHOT-shaded.jar $(echo ${PORT}) $(echo ${DELAY})
