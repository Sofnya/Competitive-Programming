#!/bin/bash
for i in  {0..6}
do
    if target/debug/is_there test/input$i.txt | cmp -s - test/output$i.txt; then
        echo test-$i passed
    else
        echo test-$i error
    fi
done
