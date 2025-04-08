#!/usr/bin/env bats

# See https://gist.github.com/mohanpedala/1e2ff5661761d3abd0385e8223e16425
# for info on why we have these set here.
set -eou pipefail

# Run this from the top level directory, i.e.,
# bats tests/client_tests.bats

# We only want to run the client once at the start because
# it takes quite a while, so the `setup` work only happens
# one time.
setup(){
    if [ "$BATS_TEST_NUMBER" -eq 1 ]; then
      pushd tests/lib || exit
      java -jar Segmented-File-System-server.jar &
      sleep 1
      popd || exit

      # Clean out any previously downloaded files.
      rm -f small.txt
      rm -f AsYouLikeIt.txt
      rm -f binary.jpg

      # Run the client
      cargo run

      kill %1
    fi
}


@test "Your client correctly assembled small.txt" {
  # Uncomment this line if you want to see the result of
  # the diff if this test is failing. Similar lines can
  # help with the other tests.
  # diff test/target-files/small.txt src/small.txt
  run diff tests/testFiles/small.txt src/small.txt

  [ "$status" -eq 0 ]
}

@test "Your client correctly assembled AsYouLikeIt.txt" {
  run diff tests/testFiles/AsYouLikeIt.txt src/AsYouLikeIt.txt

  [ "$status" -eq 0 ]
}

@test "Your client correctly assembled binary.jpg" {
  run diff tests/testFiles/binary.jpg src/binary.jpg

  [ "$status" -eq 0 ]
}