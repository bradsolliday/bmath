
var brute = function (n, m) {
	var divisor = n < m ? n : m;
	while (0 !== n % divisor || 0 !== m % divisor) {
		divisor--;
	}
	return divisor;
}

var euclid = function (n, m) {
	if (n > m) {
    	n = n % m;
        if (n === 0) {return m;}
    }
    while (true) {
        m = m % n;
        if (m === 0) {return n;}
        n = n % m;
        if (n === 0) {return m;}
    }
};

// takes as argument an integer
// returns true if it is a positve int, false otherwise
var is_positive_int = function (n, pd) {
	if (!Number.isInteger(n)) {
		$(pd).append($("<p>").text(n + " is not an integer. Please enter an integer instead."));
		return false;
	}
	if (n <= 0) {
		$(pd).append($("<p>").text(n + " is not positive. Please enter a positive integer instead."));
		return false;
	}
	return true;
}

var print_n_m = function (n, m, pd) {
	var $row = $("<tr>");
	$row.append($("<td>").text(n));
	$row.append($("<td>").text(m));
	$(pd).append($row);
}

var print_m_n_r = function (m, n, r, pd) {
	var $row = $("<tr>");
	$row.append($("<td>").text(m));	
	$row.append($("<td>").text(n));
	$row.append($("<td>").text(r));
	$(pd).append($row);
}

// pd is short for print destination
var euclid_with_print = function (n, m, pd) {
	var $header = $("<tr>");
	$header.append($("<th>").text("n"));
	$header.append($("<th>").text("m"));
	$(pd).append($header);
	print_n_m(n, m, pd);
	if (n > m) {
    	n = n % m;
    	print_n_m(n, m, pd);
        if (n === 0) {return m;}
    }
    while (true) {
        m = m % n;
        print_n_m(n, m, pd);
        if (m === 0) {return n;}
        n = n % m;
        print_n_m(n, m, pd);
        if (n === 0) {return m;}
    }
};

var simple_euclid = function (m, n, pd) {
	var $header = $("<tr>");
	$header.append($("<th>").text("m"));
	$header.append($("<th>").text("n"));
	$header.append($("<th>").text("r"));
	$(pd).append($header);
	var r = m % n;
	print_m_n_r(m, n, r, pd);
	while (r != 0) {
		m = n;
		n = r;
		r = m % n;
		print_m_n_r(m, n, r, pd);
	}
	return n;
}

var call_euclid = function () {
	$("#demo2-output").empty();
	var n = Number($("#demo2-input .input-1").val());
	var m = Number($("#demo2-input .input-2").val());
	if (!is_positive_int(n, "#demo2-output") || !is_positive_int(m, "#demo2-output")) {
		return;
	}
	var gcd;
	if ($("#demo2-input .print-steps").is(":checked")) {
		$("#demo2-output").append($("<table>"));
		gcd = euclid_with_print(n, m, "#demo2-output table");
	} else {
		gcd = euclid(n, m);
	}
	$("#demo2-output").append($("<p>").text("The gcd of " + n + " and " + m + " is " + gcd));
};

var call_brute = function () {
	$("#demo-brute-output").empty();
	var n = Number($("#demo-brute-input .input-1").val());
	var m = Number($("#demo-brute-input .input-2").val());
	if (!is_positive_int(n, "#demo-brute-output") || !is_positive_int(m, "#demo-brute-output")) {
		return;
	}
	if (n > 1000000000 && m > 1000000000) {
		$("#demo-brute-output").append($("<p>").text("Ok, maybe those values are a bit too large. Keep at least one of your values less than 1000000001."));
		return;
	}
	var gcd = brute(n, m);
	$("#demo-brute-output").append($("<p>").text("The gcd of " + n + " and " + m + " is " + gcd));
}

var call_simple_euclid = function () {
	$("#demo1-output").empty();
	var m = Number($("#demo1-input .input-1").val());
	var n = Number($("#demo1-input .input-2").val());
	if (!is_positive_int(n, "#demo1-output") || !is_positive_int(m, "#demo1-output")) {
		return;
	}
	$("#demo1-output").append($("<table>"));
	var gcd = simple_euclid(n, m, "#demo1-output table");
	$("#demo1-output").append($("<p>").text("The gcd of " + n + " and " + m + " is " + gcd));
}

var main = function() {
	"use strict";

	$("#demo2-input #calculate-gcd").on("click", call_euclid);

	$("#demo2-input #clear-output").on("click", function () {
		$("#demo2-output").empty();
	});

	$("#demo-brute-input #calculate-gcd").on("click", call_brute);

	$("#demo-brute-input #clear-output").on("click", function () {
		$("#demo-brute-output").empty();
	});

	$("#demo1-input #calculate-gcd").on("click", call_simple_euclid);

	$("#demo1-input #clear-output").on("click",function () {
		$("#demo1-output").empty();
	});
};

$(document).ready(main);