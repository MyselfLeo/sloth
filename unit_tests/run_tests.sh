# This file compiles the current build using cargo in release mode, then runs each unit tests

BLUE="\033[34m"
GREEN="\033[32m"
RED="\033[31m"
NC="\033[0m"


echo -e $BLUE "-  BUILDING SLOTHLANG" $NC

# Send warnings to /dev/null
cargo build --release --target-dir unit_tests 2>/dev/null || exit 1

echo -e $BLUE "   BUILDING COMPLETE" $NC

cd unit_tests

TEST_COUNT=0
SUCCESS=0

echo -e $BLUE "-  RUNNING UNIT TESTS" $NC

for FILENAME in *.slo
do
    TEST_COUNT=$(expr $TEST_COUNT + 1)
    RESULT=$(./release/slothlang $FILENAME)

    if [ $? -eq 0 ]; then
        SUCCESS=$(expr $SUCCESS + 1)
        echo -e "    " $GREEN $FILENAME: $NC"SUCCESS"
    else
        echo -e "    " $RED $FILENAME: $NC$RESULT
    fi
done

echo -e $BLUE "-  RESULTS:" $(expr $SUCCESS \* 100 / $TEST_COUNT)% $NC