jQuery.extend( jQuery.fn.dataTableExt.oSort, {
    "productTitle-pre": function ( a ) {
        var fragment = $('<div/>').append(a);
        var text     = fragment.children('a').find('b').text().toLowerCase();
        var x        = text.replace(/[*"]/,'');

        return x;
    },
    "productTitle-asc": function ( a, b ) {
        console.log(a+" ~~ "+b);
        return ((a < b) ? -1 : ((a > b) ? 1 : 0));
    },
    "productTitle-desc": function ( a, b ) {
        console.log(a+" ~~ "+b);
        return ((a < b) ? 1 : ((a > b) ? -1 : 0));
    }
});