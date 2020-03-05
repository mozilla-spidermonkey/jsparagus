
;
var YAHOO = YAHOO || {};
YAHOO.JP = YAHOO.JP || {};
YAHOO.JP.trv = YAHOO.JP.trv || {};
YAHOO.JP.trv.ls = YAHOO.JP.trv.ls || {};

YAHOO.JP.trv.ls.utli = {
    retryMax   : 10
   ,retryTimer : 100
   ,retryCount : 0
   ,loadJS : function(ns, callback){
        if(YAHOO.JP.trv.ls.utli.retryCount >= YAHOO.JP.trv.ls.utli.retryMax){
            YAHOO.JP.trv.ls.utli.retryCount=0;
            return;
        }
        YAHOO.JP.trv.ls.utli.retryCount++;
        if(YAHOO.JP.trv.ls[ns]){
            YAHOO.JP.trv.ls.utli.retryCount = 0;
            callback();
        }else{
            setTimeout(function(){
                YAHOO.JP.trv.ls.utli.loadJS(ns, callback);
            },YAHOO.JP.trv.ls.utli.retryTimer);
        }
    }
};

