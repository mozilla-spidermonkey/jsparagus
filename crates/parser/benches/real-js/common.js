/*
FLASH 삽입 스크립트
*/
function set_Embed()
{
  var obj = new String;
  var parameter = new String;
  var embed = new String;
  var html = new String;
  var allParameter = new String;
  var clsid = new String;
  var codebase = new String;
  var pluginspace = new String;
  var embedType = new String;
  var src = new String;
  var width = new String;
  var height = new String;
  var ServerIp = new String;
  var UserId = new String;
  var PassiveMode = new String;
  var Port = new String;
  var Status = new String;
  var Banner = new String;
  var ECHosting = new String;
  var FilelinkService = new String;
  var FilelinkServer = new String;

  this.init = function( s ,w , h, getType ) {
    getType = (getType != undefined)? getType :'flash';
    if ( getType == "flash")
    {
    clsid = "D27CDB6E-AE6D-11cf-96B8-444553540000";
    codebase = "http://download.macromedia.com/pub/shockwave/cabs/flash/swflash.cab#version=6,0,29,0";
    pluginspage = "http://www.macromedia.com/go/getflashplayer";
    embedType = "application/x-shockwave-flash";

    parameter += "<param name='movie' value='"+ s + "'>\n";
    parameter += "<param name='quality' value='high'>\n";
    parameter += "<Param name='bgcolor' value=#FFFFFF>\n";
    }
    else if ( getType == 'webftp')
    {
        clsid = "EF256D78-3982-4F12-900B-AD8B254A43BD";
        codebase = "http://echosting.cafe24.com/ftpclient/Cafe24FtpCtl21.cab#version=1,0,2,7";
    }
    else if ( getType == 'filelinkftp')
    {
        clsid = "EF256D78-3982-4F12-900B-AD8B254A43BD";
        codebase = "http://echosting.cafe24.com/ftpclient/Cafe24FtpCtl14.cab#version=1,0,2,4";
    }

        src = s;
        width = w;
        height = h;
    }

    this.parameter = function( parm , value ) {
        parameter += "<param name='"+parm +"' value='"+ value + "'>\n";
        allParameter += " "+parm + "='"+ value+"'";
    }
    
    this.show = function(getType) {
        if ( clsid)
        {
            obj = "<object classid=\"clsid:"+ clsid +"\" codebase=\""+ codebase +"\"";
    
            if (width) {
                obj += " width ='" + width + "' ";
        }
    
        if (height) {
            obj += " height ='" + height + "' ";
        }
    
        obj += ">\n";
        }
    
        if ( getType == "flash" || typeof(getType) == "undefined") {
        embed = "<embed src='" + src + "' pluginspage='"+ pluginspage + "' type='"+ embedType + "'";
    
        if (width) {
            embed += " width ='" + width + "' ";
        }
    
        if (height) {
            embed += " height ='" + height + "' ";
        }
    
        embed += allParameter + " ></embed>\n";
        }
    
    if (getType == 'streaming')
        {
        embed = "<embed src='" + src + "' type='"+ embedType + "'";
    
        if (width) {
            embed += " width ='" + width + "' ";
        }
    
        if (height) {
            embed += " height ='" + height + "' ";
        }
    
        embed += allParameter + " ></embed>\n";
        }
    
        if ( obj )
        {
        end_embed = "</object>\n";
        }
    
        if (getType == 'streaming')
            html = embed;
        else
            html = obj + parameter + embed + end_embed;
    
        document.write( html );
    }

}