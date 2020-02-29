var ErrorTracking = function (loggerLevel, debug) {
    var _debug = debug || false;
    var _loggerLevel = loggerLevel || 'error';
    this.init = function (name) {
        window.onerror = function (errorMsg, url, lineNumber, column) {
            var JSONObj = {};
            JSONObj.errorMsg = errorMsg;
            JSONObj.urlScript = encodeURIComponent(url);
            JSONObj.lineNumber = lineNumber;
            JSONObj.column = column;
            JSONObj.userAgent = navigator.userAgent;
            JSONObj.hostname = window.location.hostname;
            JSONObj.href = encodeURIComponent(window.location.href);
            JSONObj.path = window.location.pathname;
            JSONObj.appName = name;
            JSONObj.adv = advempid || null;
            logError(JSONObj);
            return true;
        };
    };
    var getDebug = function(){
        return _debug;
    };
    var getLoggerLevel = function(){
        return _loggerLevel;
    }
    var logError = function (jsonData) {
        sendData(jsonData, loggerLevel);
    };
    var sendData = function (jsonData, loggerLevel) {
        var img = document.createElement('img');
        img.src = trkUrl + "jsLogger.php?data=" + JSON.stringify(jsonData) + "&loggerLevel=" + getLoggerLevel();
        img.width = '1px';
        img.height = '1px';
        img.position = 'absolute';
        img.top = '-4000px';
        img.left = '-4000px';
        document.body.appendChild(img);
    };
};