window.tvcast=void 0!==window.tvcast?window.tvcast:{},window.tvcast.pc=void 0!==window.tvcast.pc?window.tvcast.pc:{},tvcast.pc.PlayTimeLogger=function(){var _playTimeIntervalId=null,_apiDomain="",_device="PC",_welRmcPlayer=null,_rmcPlayer=null,_calcPlayTimeThreshold=9,_lastPlayTime=0,_storageId="tvcast.playTimeLog",_isBeforeSendLog=!1,_playTimeLog=null;function _getCurrentGmt9Time(){var localDate=new Date;return localDate.getTime()+6e4*localDate.getTimezoneOffset()+324e5}function _toFixed(toFix){return null==toFix||isNaN(toFix)?0:toFix.toFixed(2)}function _sendLog(){var playTimeLogJson,playTimeLog="undefined"!==(playTimeLogJson=window.localStorage.getItem(_storageId))?$Json(playTimeLogJson).toObject():null;(function(playTimeLog){return null!=playTimeLog&&0<playTimeLog.length&&0<playTimeLog.playTime&&0<playTimeLog.lastOffset})(playTimeLog)&&0==_isBeforeSendLog&&(clearInterval(_playTimeIntervalId),function(playTimeLog){var ajaxUrl=_apiDomain+"/api/open/playtimelog/"+playTimeLog.clipNo+"?playlist="+playTimeLog.playlistNo+"&length="+_toFixed(playTimeLog.length)+"&startTimestamp="+playTimeLog.startTimestamp+"&endTimestamp="+playTimeLog.endTimestamp+"&playTime="+_toFixed(playTimeLog.playTime)+"&lastOffset="+_toFixed(playTimeLog.lastOffset)+"&device="+_device+"&rcTimestamp="+playTimeLog.rcTimestamp;$Ajax(ajaxUrl,{type:"jsonp",callbackname:"callback",onload:function(res){},onerror:function(res){}}).request()}(playTimeLog),window.localStorage.removeItem(_storageId),_isBeforeSendLog=!0)}function isAbleLocalStorage(){return"undefined"!=typeof Storage}function _clearLogPlayTimeInterval(){null!=_playTimeIntervalId&&clearInterval(_playTimeIntervalId)}function _logPlayTimeInterval(){_playTimeLog.length=function(){try{return null!=_welRmcPlayer?_welRmcPlayer.getVideoTimes()[1]:null!=_rmcPlayer?_rmcPlayer.getDuration():0}catch(e){return 0}}();var playTimeLogJson,playTime=function(){try{return null!=_welRmcPlayer?_welRmcPlayer.getVideoTimes()[0]:null!=_rmcPlayer?_rmcPlayer.getCurrentTime():0}catch(e){return 0}}(),elapseTime=playTime-_lastPlayTime;0<elapseTime&&elapseTime<=_calcPlayTimeThreshold&&(_playTimeLog.playTime+=elapseTime),_lastPlayTime=playTime,_playTimeLog.lastOffset<playTime&&(_playTimeLog.lastOffset=playTime),_playTimeLog.endTimestamp=_getCurrentGmt9Time(),playTimeLogJson=$Json(_playTimeLog).toString(),window.localStorage.setItem(_storageId,playTimeLogJson)}function _init(oOptions){if(_clearLogPlayTimeInterval(),_isBeforeSendLog=!1,_playTimeLog={clipNo:"",playlistNo:"",length:0,startTimestamp:null,endTimestamp:null,playTime:0,lastOffset:0,rcTimestamp:null},"undefined"===oOptions.apiDomain||"undefined"===oOptions.clipNo||null==oOptions.welRmcPlayer&&null==oOptions.rmcPlayer)return!1;if(!isAbleLocalStorage())return!1;if(_apiDomain=oOptions.apiDomain,null!=oOptions.welRmcPlayer){if(function(welRmcPlayer){if("function"!=typeof welRmcPlayer.getVideoTimes)return!0;var videoTimes=welRmcPlayer.getVideoTimes();return null==videoTimes||videoTimes.length<2}(oOptions.welRmcPlayer))return!1;_welRmcPlayer=oOptions.welRmcPlayer}if(null!=oOptions.rmcPlayer){if("function"!=typeof(rmcPlayer=oOptions.rmcPlayer).getCurrentTime||"function"!=typeof rmcPlayer.getDuration)return!1;_rmcPlayer=oOptions.rmcPlayer}var rmcPlayer,timestamp;return _playTimeLog.clipNo=oOptions.clipNo,_playTimeLog.playlistNo=oOptions.playlistNo,_playTimeLog.rcTimestamp=(timestamp=window.sessionStorage.getItem("rc"),window.sessionStorage.removeItem("rc"),timestamp),!0}return{logPlayTime:function(oOptions){_init(oOptions)&&(_sendLog(),_playTimeLog.startTimestamp=_getCurrentGmt9Time(),_clearLogPlayTimeInterval(),_playTimeIntervalId=setInterval(_logPlayTimeInterval,3e3))},sendLog:function(){if(!isAbleLocalStorage())return!1;_isBeforeSendLog=!1,_sendLog()},clearInterval:function(){_clearLogPlayTimeInterval()}}}();