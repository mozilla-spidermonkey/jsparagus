assertEq(true, true);

assertEq(1 < 2, true);
assertEq(2 < 1, false);

assertEq(undefined ?? "abc", "abc");

var x = 1;
if (1 < 100) {
	x = 42;
} else {
	x = 12;
}
assertEq(x, 42)

assertEq(Math.sqrt(36), 6);
