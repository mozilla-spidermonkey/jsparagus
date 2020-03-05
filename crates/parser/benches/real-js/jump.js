/*
 * @name: jump.js
 * @description: 接收子页面的message进行跳转
 * @author: 一血
 * @date: 2016-9-21
 */
(function(window) {
  var onmessage = function(event) {
    var data = event.data;
    if(data.redirectURL){
      location.href = data.redirectURL;
    }
  };
  if (typeof window.addEventListener !== 'undefined') {
    window.addEventListener('message', onmessage, false);
  } else if (typeof window.attachEvent !== 'undefined') {
    //for ie
    window.attachEvent('onmessage', onmessage);
  }
})(window);
