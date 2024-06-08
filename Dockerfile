# Use the official Swift image.
# https://hub.docker.com/_/swift
FROM swift:5.9

# Copy local code to the container image.
WORKDIR /app
COPY . .

# Install dependencies and build.
RUN swift build -c release

# Run the web service on container startup.
CMD [ ".build/release/sweetrpg-kv-objects"]
