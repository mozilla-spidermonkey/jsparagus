(function() {
    var userAgent = navigator.userAgent.toLowerCase();
    if (/mobile/i.test(userAgent) && /ipad/i.test(userAgent) === false) {
        if (location.search) {
            if (location.search.indexOf('noTitleBar=1') !== -1) {
                window.location.href = '/m' + location.pathname + location.search;
            } else {
                window.location.href = '/m' + location.pathname + location.search + '&noTitleBar=1';
            }
        } else {
            window.location.href = '/m' + location.pathname + '?noTitleBar=1';
        }
        return;
    }
})();
