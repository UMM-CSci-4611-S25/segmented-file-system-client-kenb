#!/usr/bin/env bats

# See https://gist.github.com/mohanpedala/1e2ff5661761d3abd0385e8223e16425
# for info on why we have these set here.
set -eou pipefail

# Run this from the top level directory, i.e.,
# bats tests/client_tests.bats

# We only want to run the client once at the start because
# it takes quite a while, so the `setup` work only happens
# one time.
setup() {
    if [ "$BATS_TEST_NUMBER" -eq 1 ]; then
        pushd tests/lib || exit

        # Check if port 7077 is free
        if lsof -i:7077; then
            echo "Port 7077 is already in use!"
            exit 1
        fi

        # Start the server in the background
        java -jar Segmented-File-System-server.jar &
        SERVER_PID=$!

        # Wait for the server to be ready
        for i in {1..10}; do
            if nc -z 127.0.0.1 7077; then
                echo "Server is ready"
                break
            fi
            sleep 1
        done

        # Clean out any previously downloaded files
        rm -f small.txt AsYouLikeIt.txt binary.jpg

        # Run the client
        cargo run

        popd || exit
    fi
}

teardown() {
    # Ensure the server process is killed after all tests
    if [ -n "${SERVER_PID:-}" ]; then
        kill $SERVER_PID
    fi
}

@test "Your client correctly assembled small.txt" {
  # Uncomment this line if you want to see the result of
  # the diff if this test is failing. Similar lines can
  # help with the other tests.
  # diff test/target-files/small.txt src/small.txt
    run diff "$(pwd)/tests/testFiles/small.txt" "$(pwd)/src/small.txt"

  [ "$status" -eq 0 ]
}

@test "Your client correctly assembled AsYouLikeIt.txt" {
    run diff "$(pwd)/tests/testFiles/AsYouLikeIt.txt" "$(pwd)/src/AsYouLikeIt.txt"

  [ "$status" -eq 0 ]
}

@test "Your client correctly assembled binary.jpg" {
    run diff "$(pwd)/tests/testFiles/binary.jpg" "$(pwd)/src/binary.jpg"

  [ "$status" -eq 0 ]
}