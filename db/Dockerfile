FROM postgres:latest

# Copy the setup script
# This will need to be ran while
# the container is running for the
# first time.
COPY ./setup-schema.sh .
RUN chmod +x ./setup-schema.sh