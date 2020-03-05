var yflocalstorage = window.yflocalstorage || {};

(function (window) {

    var MAX_RECOMMEND_LENGTH = 10;
    var HISTORY_KEY   = "history_fortune";
    var RECOMMEND_KEY = "recommend_fortune";
    var storage = window.localStorage;

    function put(key, obj) {
        if (obj !== undefined && obj !== null && typeof obj === 'object') {
            storage.setItem(key, JSON.stringify(obj));
        }
    }

    function get(key) {
        var res = JSON.parse(storage.getItem(key));
	    if (res) {
	        return res; 
	    } else {
            return [];
        }
    }

    function removeLast(arr) {
        if (arr.length > 1) {
            arr.pop(); 
            return true;
        }
	return false;
    }

    function add(arr, item) {
        if(arguments.length > 1) {
            arr.unshift(item); 
        }
    }

    function getCid() {
        try {
            var cnl = document.getElementsByName('channel')[0].value;
            if(cnl !== null && typeof cnl === 'string' && cnl !== '') {
                var addNum = "1";
                while(true) {
                    var res = addNum + cnl.toString();
                    if(res.length === 10) {
                        return res;
                    } else {
                        addNum = addNum + "0";
                    }
                }
            }
        } catch(e) {}
    }

    function checkItemList(list, cid) {

        var list = list ? list : [];
        
        if(list.indexOf(cid) < 0) {
            return true; 
        } else {
            return false; 
        }
    }

    function init() {

        var cid = getCid();
        var domain = (location.protocol==="https:") ? "https://charge-fortune.yahoo.co.jp":"http://charge.fortune.yahoo.co.jp";
        var url = domain + "/bin/get_recommend.php"

        if ( cid === null || typeof cid !== 'string' || !cid.match(/^\w{10}$/) ) {
            return new Error("cid is not null");
        }

        var histItemList = get(HISTORY_KEY);
        var cidList = [];
        if(histItemList != null && typeof histItemList === 'object' && histItemList.length !== 0) {

            cidList = getCidList(histItemList);
        }

        if (checkItemList(cidList, cid)) {

            if (cidList !== null && cidList.length >= MAX_RECOMMEND_LENGTH) {

                if (removeLast(cidList)) {
                    add(cidList, cid);
                }

            } else {

                add(cidList, cid);
            }
             
        }

        $.ajax({
            url: url,
            type: "POST",
            dataType: "json",
            data: {"history_cid": cidList, "cid": cid},
            success: function(data, textStatus, xhr) {
                var historyData = $.map(data['history_data'], function(value, index) {
                    return value; 
                });  
                var recommendData = data['recommend_data'];  


                if (historyData !== null && typeof historyData === 'object' && historyData.length > 0) {
                    put(HISTORY_KEY, historyData);
                }

                if (recommendData !== null && typeof recommendData === 'object' && recommendData.length > 0) {
                    put(RECOMMEND_KEY, recommendData);
                }


            },

        });

    }

    function getCidList(aryList) {

        var aryCidList = [];

        for(var i = 0; i < aryList.length; i++) {
            aryCidList.push(aryList[i]['cid']); 
        }

        return aryCidList;
    }

    (function () {
        if (window.localStorage) {

            window.yflocalstorage = {
                put: put,
                get: get,
                removeLast: removeLast,
                add: add,
                init: init,
            };
        } else {
            throw new Error('This browser have no storage.');
        }
    }());


}(this));

if(typeof jQuery == "undefined") {
    var script = document.createElement('script');
    script.src = "https://s.yimg.jp/images/fortune/js/yjquery.min.js";
    script.onload = function() {
        yflocalstorage.init();
    }
    document.body.appendChild(script);
} else {
    yflocalstorage.init();
}
