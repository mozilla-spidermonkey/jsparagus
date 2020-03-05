jQuery.extend( jQuery.fn.dataTableExt.oSort, {
    "publisher-pre": function ( a ) {
        var fragment = $(a);
        var x        = fragment.text();

        return x;
    },
    "publisher-asc": function ( a, b ) {
        return ((a < b) ? -1 : ((a > b) ? 1 : 0));
    },
    "publisher-desc": function ( a, b ) {
        return ((a < b) ? 1 : ((a > b) ? -1 : 0));
    }
});