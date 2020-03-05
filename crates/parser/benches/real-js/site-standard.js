function multipleBgTest(){

  var el_style = document.createElement("div").style;
  el_style.cssText = 'background:url(//:),url(//:),red url(//:)';

  // If the UA supports multiple backgrounds, there should be three occurrences
  // of the string "url(" in the return value for el_style.background
  return new RegExp("(url\\s*\\(.*?){3}").test(el_style.background);

}

function popupWindow(url, name) {
// Provide a name if undefined.
if (typeof name == 'undefined' || name.length == 0) {
  name = 'popupWindow';
}
// Open the window.
window.open(url,name,'toolbar=yes,location=yes,directories=no,status=no,menubar=no,scrollbars=no,resizable=yes,copyhistory=no,width=400,height=300,screenX=150,screenY=150,top=150,left=150')
}

function popupWindow(url, width, height) {
    window.open(url,'popupWindow','toolbar=no,location=no,directories=no,status=no,menubar=no,scrollbars=no,resizable=no,copyhistory=no,width='+width+',height='+height+',screenX=150,screenY=150,top=150,left=150')
}

function popupWindowSized(url, width, height, name) {
// Provide a name if undefined.
if (typeof name == 'undefined' || name.length == 0) {
  name = 'popupWindow';
}
// Open the window.
window.open(url,name,'toolbar=yes,location=yes,directories=no,status=no,menubar=no,scrollbars=no,resizable=yes,copyhistory=no,width='+width+',height='+height+',screenX=150,screenY=150,top=150,left=150')
}

function addLoadEvent(func) {
  var oldonload = window.onload;
  if (typeof(window.onload) != 'function') {
    window.onload = func;
  } else {
    window.onload = function() {
      if (oldonload) {
        oldonload();
      }
      func();
    }
  }
}

function sz(t) {
	// makes a textarea box expand as you add text.
	a = t.value.split('\n');
	b=0;
	for (x=0;x < a.length; x++) {
		if (a[x].length >= t.cols) b+= Math.floor(a[x].length/t.cols);
	}
	b+= a.length;
	if (b > t.rows) t.rows = b;
}

function textCounter(field, countfield, maxlimit) {
	if (field.value.length > maxlimit) // if too long...trim it!
		field.value = field.value.substring(0, maxlimit);
	// otherwise, update 'characters left' counter
	else
		countfield.value = maxlimit - field.value.length;
}

function showRecaptcha(element) {
	Recaptcha.create('6Le17goAAAAAAOC01JUReIPm-3XqGxeSGa6J5zQH', element, {
		theme: "yellow",
		callback: Recaptcha.focus_response_field});
}

// Verhoeff algorithm get & set, by Avraham Plotnitzky. (aviplot at gmail)
// A more advanced credit card number validation algorithm
var Verhoeff = {
    "d":[[0,1,2,3,4,5,6,7,8,9],
        [1,2,3,4,0,6,7,8,9,5],
        [2,3,4,0,1,7,8,9,5,6],
        [3,4,0,1,2,8,9,5,6,7],
        [4,0,1,2,3,9,5,6,7,8],
        [5,9,8,7,6,0,4,3,2,1],
        [6,5,9,8,7,1,0,4,3,2],
        [7,6,5,9,8,2,1,0,4,3],
        [8,7,6,5,9,3,2,1,0,4],
        [9,8,7,6,5,4,3,2,1,0]],
    "p":[[0,1,2,3,4,5,6,7,8,9],
        [1,5,7,6,2,8,3,0,9,4],
        [5,8,0,3,7,9,6,1,4,2],
        [8,9,1,6,0,4,3,5,2,7],
        [9,4,5,3,1,2,6,8,7,0],
        [4,2,8,6,5,7,3,9,0,1],
        [2,7,9,3,8,0,6,4,1,5],
        [7,0,4,6,9,1,3,2,5,8]],
    "j":[0,4,3,2,1,5,6,7,8,9],
    "check":function(str)
    {
        var c = 0;
        str.replace(/\D+/g,"").split("").reverse().join("").replace(/[\d]/g, function(u, i, o){
            c = Verhoeff.d[c][Verhoeff.p[i&7][parseInt(u,10)]];
        });
        return (c === 0);

    }
}

//JavaScript based implementation of Luhn credit card number validation
var Luhn = {
 // Javascript code copyright 2009 by Fiach Reid : www.webtropy.com
 // This code may be used freely, as long as this copyright notice is intact.
    Calculate: function(Luhn) {
        var sum = 0;
        for (i=0; i<Luhn.length; i++ )
        {
            sum += parseInt(Luhn.substring(i,i+1));
        }
        var delta = new Array (0,1,2,3,4,-4,-3,-2,-1,0);
        for (i=Luhn.length-1; i>=0; i-=2 )
        {
            var deltaIndex = parseInt(Luhn.substring(i,i+1));
            var deltaValue = delta[deltaIndex];
            sum += deltaValue;
        }
        var mod10 = sum % 10;
        mod10 = 10 - mod10;
        if (mod10==10)
        {
            mod10=0;
        }
        return mod10;
     },
     Validate: function(Luhn) {
         var LuhnDigit = parseInt(Luhn.substring(Luhn.length-1,Luhn.length));
         var LuhnLess = Luhn.substring(0,Luhn.length-1);
         if (Calculate(LuhnLess)==parseInt(LuhnDigit)) {
             return true;
         }
         return false;
     }
}

//Object to manage AJAX interactions on a page.
//Given a path to an AJAX script it will queue calls to that script using
//the schedule() command.
function AjaxQueue() {
    var _config = {};
    var _cbQueue = [];
    var _mDefer = null;
    var _formLock = false;

    function init(opts) {

        _config = {
            delay: 500,
            type: 'GET',
            target: '',
            uiElement: ''
        };
        $.extend(true,_config,opts);
    }

    function schedule(params) {

        ajx = {
            data:{},
            cbScs: {}
        };
        $.extend(true,ajx,params);

        _cbQueue.push(
            $.ajax({
               type: _config.type,
               url: _config.target,
               data: ajx.data,
               success: ajx.cbScs
        }));

        _queueAjax();
    }

    function isProcessing() {
        return _formLock;
    }

    function _queueAjax() {
        _formLock = true;
        _uiWarn();
        if(_mDefer) clearTimeout(_mDefer);
        _mDefer = setTimeout(function() {
            $.when.apply($,_cbQueue).done( function() { _cleanup(); })
        },_config.delay);
    }

    function _uiWarn() {
        if(_config.uiElement) $(_config.uiElement).fadeTo('fast',0.5);
    }

    function _cleanup() {
        clearTimeout(_mDefer);
        if(_config.uiElement) $(_config.uiElement).fadeTo('fast',1);
        _formLock = false;
        _mDefer = null;
        _cbQueue = [];
    }

    return {
        init: init,
        schedule: schedule,
        isProcessing: isProcessing
    }
}

//Object to dispense REST tokens
function KeyMaster() {
    var _config = {};

    function init(opts) {

        _config = {
            type: 'GET',
            endpoint: '',
	    data: {
                request_type: 'nothing',
                //currently product_request for a customer request and publisher_request for pub tools requests
                target: 0,
                //should be the most specific identifier for the request - an order id, product id or what have you
                request_contents: []
                //related request data should arrive in decreasing order of specificity - for a product it would be
                //orders products download id followed by bundle index.  See API endpoint routes for details
	    }
        };
        $.extend(true,_config,opts);
    }

    function token(params) {

        var data = _config.data;
        $.extend(true,data,params);

        return $.ajax({
                   type: _config.type,
                   url: _config.endpoint,
                   data: data
               });
    }

    return {
        init: init,
        token: token
    }
}

function DownloadStatus(opts) {

    var _config = {
            type: 'GET',
            uri_base: '',
            site: '',
            customer_id: '',
            mfr_id: '',
            order_id: '',
            opd_id: '',
            bidx: '',
            timeout: 20000,
            updateElement: '',
            tkn: '',
            test: '',
            callback: ''
    };
    $.extend(true,_config,opts);

    var _endpoint = '';
    var _response_data;

    function _message(txt,value) {
        _config.updateElement.find('.status').html(txt);
    };

    function _setup() {

        for(var setting in _config) {
            if(_config.hasOwnProperty(setting)) {
                if(setting == '') {
                    _message('Cannot get status of this download - missing order information');
                }
            }
        }
        _endpoint = _config.uri_base + '/' + _config.order_id + '/' + _config.opd_id + '/' + _config.bidx;

        if(_config.tkn === '') {
            var _token_dispensory = new KeyMaster();
            var token = _token_dispensory.token({
                            request_type: 'product_request',
                            target: _config.order_id,
                            request_contents: [ _config.opd_id, _config.bidx ]
                        });
            $.when(token).
            done(function(resp) {
                _config.tkn = resp.token;
                _poll();
            }).
            fail(function(resp) {
                _message('Could not retrieve a status update');
            });
        } else {
            _poll();
        }
    };

    function _poll() {

        $.ajax({
            url: _endpoint,
            type: _config.type,
            data: {
                cid  : _config.customer_id,
                mid  : _config.mfr_id,
                site : _config.site,
                tkn  : _config.tkn
            },
            dataType: 'json',
            timeout: _config.timeout
        }).
        done(function(resp) {
            _response_data = resp;

            _message(_response_data.status, _response_data.progress);

            if(resp.status == 'Complete') {
                if(typeof _config.callback === 'function')
                    _config.callback();
            } else {
                setTimeout( _poll, _config.timeout );
            }
        }).
        fail(function(xhr) {
            _response_data = {error: xhr.responseText};
            _message(_response_data.error);
        });

    };

    _setup();

}

function parse_url_query(href) {

    var parser = document.createElement('a');
    parser.href = href;

    var query_string = parser.search;
    var query_array  = query_string.replace(/\?/gi,'').split('&');
    var hash = {};

    query_array.forEach(function(elem,index,array) {
        var data = elem.split('=');
        hash[ data[0] ] = data[1];
    });

    return hash;

}

function updateCartWidget(qty) {
    var cart_widget = $('#header-cart-widget');
    var cart_widget_counter = $('#cart-widget-qty');

    if(qty === undefined || qty === null) return;

    if(qty > 0) {
        cart_widget_counter.text(qty);
        if(!cart_widget.is(':visible')) cart_widget.fadeIn('fast');

    } else {
        cart_widget.fadeOut('fast');
        cart_widget_counter.text(qty);
    }
}

function updateWishWidget(qty) {
    // DEPR. With the multiple wishlists, we no longer need to track
    // # of products in a list. Func left pending removal from all code.
    // - SeH 02.28.14
}

/**
 * Helper function to tell if a value is null by PHP standards.
 */
function isNull(value) {
	if ((value === null) || typeof value == 'undefined' || value.length == 0) {
		return true;
	}
	return false;
}

/**
 * Helper function to tell if a value is empty by PHP standards.
 */
function isEmpty(value) {
	if (isNull(value) || value == '' || value == 0 || value == false) {
		return true;
	}
	return false;
}

/**
 * Generates the HTML to create a "spinner" icon. Uses font-awesome icons.
 *
 * @param size 			Size relative to container. lg, 2x, 3x, 4x, 5x, etc. See https://fortawesome.github.io/Font-Awesome/examples/#larger
 * @param spinner_type	Spinner type. "spinner", "circle-o-notch", "refresh", or "cog". See https://fortawesome.github.io/Font-Awesome/examples/#animated
 * @param animation		Animation type. "pulse" (best for spinner class), or "spin" (best for all others). See https://fortawesome.github.io/Font-Awesome/examples/#animated
 * @param additional_params Any additional parameters to include in the <i> element.
 * @param classes	Additional css classes
 *
 * @return string	String representing the output HTML for the spinner.
 */
function create_spinner_html(size_class, spinner_type, animation_type, additional_params, classes) {
	spinner = (spinner_type === undefined || spinner_type === '' || spinner_type === null) ? 'spinner' : spinner_type;

	animation = (animation_type === undefined || animation_type === '' || animation_type === null) ? 'pulse' : animation_type;

	size = (size_class === undefined || size_class === '' || size_class === null) ? '' :  'fa-'.$size_class;

	params = (additional_params === undefined || additional_params === '' || additional_params === null) ? '' : additional_params;

	cssClass = (classes === undefined || classes === '' || classes === null) ? '' : classes;

	output = "<i style='text-align: center;' class=\"fa fa-{$spinner_type} fa-{$animation}{$size_class} {$classes}\" {$additional_params}></i>"
			.replace('{$spinner_type}', spinner)
			.replace('{$animation}', animation)
			.replace('{$size_class}', size)
			.replace('{$additional_params}', params)
			.replace('{$classes}', cssClass); // Requires font-awesome

	return output;
}

// Replace alert() funcs with a small popup div.
function obsAlert(message, dismissMessage) {
  if (isEmpty(dismissMessage)) { dismissMessage = 'Dismiss'; }
  var alertDiv = $('#obs-alert-window');
  if (isEmpty(alertDiv)) {
    var alertHtml =
      '<div id="obs-alert-window" class="obs-alert-window"><div class="box">' +
        '<div class="button">' +
          '<i class="fa fa-window-close fa-2x close" aria-hidden="true"></i>' +
        '</div>' +
        '<div class="bodybox">' +
          '<i class="fa fa-exclamation-triangle fa-3x" aria-hidden="true"></i>' +
          '<p class="message"></p>' +
        '</div>' +
        '<div class="dismiss close">' +
          '<p><a href="#">' + dismissMessage + '</a></p>' +
        '</div>' +
      '</div></div>';
    $('body').append(alertHtml);
    alertDiv = $('#obs-alert-window');
  }
  $('#obs-alert-window .message').html(message);
  $('#obs-alert-window').show();
  $('#obs-alert-window .close').click(function(){
    $('#obs-alert-window').hide();return false;
  });
  $(document).keydown(function(e) { 
    if (e.which == 27) { $('#obs-alert-window').hide(); }
  });
//  $(document).click(function() {
//    $('#obs-alert-window').hide();return false;
//  });
  $("#obs-alert-window .box").click(function(e) {
    e.stopPropagation();
  });
}

// Replace alert() funcs with a small popup div.
function obsConfirm(message, okMessage, closeMessage) {
  if (isEmpty(okMessage)) { okMessage = 'Ok'; }
  if (isEmpty(closeMessage)) { closeMessage = 'Close'; }
  var alertDiv = $('#obs-confirm-window');
  if (isEmpty(alertDiv)) {
    var alertHtml =
      '<div id="obs-confirm-window" class="obs-alert-window"><div class="box">' +
        '<div class="button">' +
          '<i class="fa fa-window-close fa-2x close" aria-hidden="true"></i>' +
        '</div>' +
        '<div class="bodybox">' +
          '<i class="fa fa-exclamation-triangle fa-3x" aria-hidden="true"></i>' +
          '<p class="message"></p>' +
        '</div>' +
        '<div class="dismiss proceed close">' +
          '<p><a href="#">' + okMessage + '</a></p>' +
        '</div>' +
        '<div class="dismiss cancel close">' +
          '<p><a href="#">' + closeMessage + '</a></p>' +
        '</div>' +
      '</div></div>';
    $('body').append(alertHtml);
    alertDiv = $('#obs-confirm-window');
  }
  $('#obs-confirm-window .message').html(message);
  $('#obs-confirm-window').show();
  $('#obs-confirm-window .close').click(function(){
    $('#obs-confirm-window').hide();return false;
  });
  $(document).keydown(function(e) { 
    if (e.which == 27) { $('#obs-confirm-window').hide(); }
  });
  $("#obs-confirm-window .box").click(function(e) {
    e.stopPropagation();
  });
}