<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_dojo.registerModulePath(tapestry, insp_b88786</name>
   <tag></tag>
   <elementGuidId>1d824288-05da-4bcc-a5cf-2b658cd5d1b2</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#Body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body[@id='Body']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>9ee7e8e2-864f-4f49-81f9-2e3c489f895d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>login_bg</value>
      <webElementGuid>28d812dd-e111-4910-987a-3359788f8975</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>Body</value>
      <webElementGuid>36c6c63c-d82e-4090-9fc8-c3a0f38a70bb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding='UTF-8';




 






Notification.requestPermission().then(function(result) {
	  if (result === 'denied') {
	    console.log('Permission wasn\'t granted. Allow a retry.');
	    return;
	  }
	  if (result === 'default') {
	    console.log('The permission request was dismissed.');
	    return;
	  }
	  // Do something with the granted permission.
});
$(document).ready(function(){
	resizeLoginTop();
});
$(window).resize(function() {
	resizeLoginTop();
});

function resizeLoginTop(){
	var htmlHeight = $('html').css('height'), marTop = 0;
	htmlHeight = Number(htmlHeight.substring(0, htmlHeight.length - 2));
	console.log(htmlHeight);
	$('#login .logo, #login .pic').css('margin-top', ((htmlHeight-30-475)/2) + 'px');
}

try{
    var exp = new Date( );
    var nowPlusOneWeek = exp.getTime( ) + (7 * 24 * 60 * 60 * 1000);
    exp.setTime(nowPlusOneWeek);
    document.cookie = &quot;tinreadMode=cata; expires=&quot; + exp.toGMTString( );
  } catch(ex) {
  }
  
//   function KeyPress(e) {
//     var keynum;
//     if(window.event) {// IE
//       keynum = e.keyCode;
//     }
//     else if(e.which) {// Netscape/Firefox/Opera
//       keynum = e.which;
//     }
//     if (keynum == 13) {
//         submitForm =dojo.byId('logon');
//         tapestry.form.submit(submitForm);
//     }
//   }
  
  function login(e) {
	  
	  document.getElementById('Submit').disabled=true;
// 	  tapestry.form.submit(e); 
	  return false;
  }
  
  function showProgressBar() {
    document.getElementById(&quot;logonForm&quot;).style.display = 'none';
    document.getElementById(&quot;progressbar&quot;).style.display  = '';
  }







&lt;!--
function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==''){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/', closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/');
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
// -->



    
        
            
                
            
            神資圖書館(15trunk機)
            
            
            






            
    		
            logon.browser.value=BrowserDetect.browser;
                
                    
                        
                           
                        
                    
                    
                        
                            
                        
                    
                    
                        
                        	
                           		
							
							
                        
                    
                
            
            
            
            	
                
            
            本網站推薦使用Chrome，最佳瀏覽解析度為1024x768以上
        
        
        
            
        
    


© 2016 艾迪訊科技股份有限公司 版權所有

  document.getElementById(&quot;member_id&quot;).focus();


  
    
 
  
  
     
  
 
 
  
 


    


 
  
    變更密碼
  
  
 
 
  
 




  
Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;logon&quot;);

closeDialogComponent('TinreadErrorDialog');
closeDialogComponent('TinreadDialog');
tapestry.form.focusField('member_id');});
// -->


id(&quot;login&quot;)/div[@class=&quot;row&quot;]/div[@class=&quot;cell&quot;]/div[@class=&quot;login_form&quot;]</value>
      <webElementGuid>878f191f-f993-4a7b-9e0d-a81594a05284</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)</value>
      <webElementGuid>25ffbc48-2567-4352-b5f6-f4ae477dd8b1</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='Body']</value>
      <webElementGuid>d02bd61c-5b66-4fea-9ced-8027f6a8b80b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>23574b14-ce64-47dd-9e0e-913362924b30</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[@id = 'Body' and (text() = concat(&quot;

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;




 






Notification.requestPermission().then(function(result) {
	  if (result === &quot; , &quot;'&quot; , &quot;denied&quot; , &quot;'&quot; , &quot;) {
	    console.log(&quot; , &quot;'&quot; , &quot;Permission wasn\&quot; , &quot;'&quot; , &quot;t granted. Allow a retry.&quot; , &quot;'&quot; , &quot;);
	    return;
	  }
	  if (result === &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;) {
	    console.log(&quot; , &quot;'&quot; , &quot;The permission request was dismissed.&quot; , &quot;'&quot; , &quot;);
	    return;
	  }
	  // Do something with the granted permission.
});
$(document).ready(function(){
	resizeLoginTop();
});
$(window).resize(function() {
	resizeLoginTop();
});

function resizeLoginTop(){
	var htmlHeight = $(&quot; , &quot;'&quot; , &quot;html&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;height&quot; , &quot;'&quot; , &quot;), marTop = 0;
	htmlHeight = Number(htmlHeight.substring(0, htmlHeight.length - 2));
	console.log(htmlHeight);
	$(&quot; , &quot;'&quot; , &quot;#login .logo, #login .pic&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;margin-top&quot; , &quot;'&quot; , &quot;, ((htmlHeight-30-475)/2) + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
}

try{
    var exp = new Date( );
    var nowPlusOneWeek = exp.getTime( ) + (7 * 24 * 60 * 60 * 1000);
    exp.setTime(nowPlusOneWeek);
    document.cookie = &quot;tinreadMode=cata; expires=&quot; + exp.toGMTString( );
  } catch(ex) {
  }
  
//   function KeyPress(e) {
//     var keynum;
//     if(window.event) {// IE
//       keynum = e.keyCode;
//     }
//     else if(e.which) {// Netscape/Firefox/Opera
//       keynum = e.which;
//     }
//     if (keynum == 13) {
//         submitForm =dojo.byId(&quot; , &quot;'&quot; , &quot;logon&quot; , &quot;'&quot; , &quot;);
//         tapestry.form.submit(submitForm);
//     }
//   }
  
  function login(e) {
	  
	  document.getElementById(&quot; , &quot;'&quot; , &quot;Submit&quot; , &quot;'&quot; , &quot;).disabled=true;
// 	  tapestry.form.submit(e); 
	  return false;
  }
  
  function showProgressBar() {
    document.getElementById(&quot;logonForm&quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    document.getElementById(&quot;progressbar&quot;).style.display  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
  }







&lt;!--
function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
// -->



    
        
            
                
            
            神資圖書館(15trunk機)
            
            
            






            
    		
            logon.browser.value=BrowserDetect.browser;
                
                    
                        
                           
                        
                    
                    
                        
                            
                        
                    
                    
                        
                        	
                           		
							
							
                        
                    
                
            
            
            
            	
                
            
            本網站推薦使用Chrome，最佳瀏覽解析度為1024x768以上
        
        
        
            
        
    


© 2016 艾迪訊科技股份有限公司 版權所有

  document.getElementById(&quot;member_id&quot;).focus();


  
    
 
  
  
     
  
 
 
  
 


    


 
  
    變更密碼
  
  
 
 
  
 




  
Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;logon&quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);
tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;member_id&quot; , &quot;'&quot; , &quot;);});
// -->


id(&quot;login&quot;)/div[@class=&quot;row&quot;]/div[@class=&quot;cell&quot;]/div[@class=&quot;login_form&quot;]&quot;) or . = concat(&quot;

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;




 






Notification.requestPermission().then(function(result) {
	  if (result === &quot; , &quot;'&quot; , &quot;denied&quot; , &quot;'&quot; , &quot;) {
	    console.log(&quot; , &quot;'&quot; , &quot;Permission wasn\&quot; , &quot;'&quot; , &quot;t granted. Allow a retry.&quot; , &quot;'&quot; , &quot;);
	    return;
	  }
	  if (result === &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;) {
	    console.log(&quot; , &quot;'&quot; , &quot;The permission request was dismissed.&quot; , &quot;'&quot; , &quot;);
	    return;
	  }
	  // Do something with the granted permission.
});
$(document).ready(function(){
	resizeLoginTop();
});
$(window).resize(function() {
	resizeLoginTop();
});

function resizeLoginTop(){
	var htmlHeight = $(&quot; , &quot;'&quot; , &quot;html&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;height&quot; , &quot;'&quot; , &quot;), marTop = 0;
	htmlHeight = Number(htmlHeight.substring(0, htmlHeight.length - 2));
	console.log(htmlHeight);
	$(&quot; , &quot;'&quot; , &quot;#login .logo, #login .pic&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;margin-top&quot; , &quot;'&quot; , &quot;, ((htmlHeight-30-475)/2) + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
}

try{
    var exp = new Date( );
    var nowPlusOneWeek = exp.getTime( ) + (7 * 24 * 60 * 60 * 1000);
    exp.setTime(nowPlusOneWeek);
    document.cookie = &quot;tinreadMode=cata; expires=&quot; + exp.toGMTString( );
  } catch(ex) {
  }
  
//   function KeyPress(e) {
//     var keynum;
//     if(window.event) {// IE
//       keynum = e.keyCode;
//     }
//     else if(e.which) {// Netscape/Firefox/Opera
//       keynum = e.which;
//     }
//     if (keynum == 13) {
//         submitForm =dojo.byId(&quot; , &quot;'&quot; , &quot;logon&quot; , &quot;'&quot; , &quot;);
//         tapestry.form.submit(submitForm);
//     }
//   }
  
  function login(e) {
	  
	  document.getElementById(&quot; , &quot;'&quot; , &quot;Submit&quot; , &quot;'&quot; , &quot;).disabled=true;
// 	  tapestry.form.submit(e); 
	  return false;
  }
  
  function showProgressBar() {
    document.getElementById(&quot;logonForm&quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    document.getElementById(&quot;progressbar&quot;).style.display  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
  }







&lt;!--
function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
// -->



    
        
            
                
            
            神資圖書館(15trunk機)
            
            
            






            
    		
            logon.browser.value=BrowserDetect.browser;
                
                    
                        
                           
                        
                    
                    
                        
                            
                        
                    
                    
                        
                        	
                           		
							
							
                        
                    
                
            
            
            
            	
                
            
            本網站推薦使用Chrome，最佳瀏覽解析度為1024x768以上
        
        
        
            
        
    


© 2016 艾迪訊科技股份有限公司 版權所有

  document.getElementById(&quot;member_id&quot;).focus();


  
    
 
  
  
     
  
 
 
  
 


    


 
  
    變更密碼
  
  
 
 
  
 




  
Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;logon&quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);
tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;member_id&quot; , &quot;'&quot; , &quot;);});
// -->


id(&quot;login&quot;)/div[@class=&quot;row&quot;]/div[@class=&quot;cell&quot;]/div[@class=&quot;login_form&quot;]&quot;))]</value>
      <webElementGuid>48ecee6d-a03f-4995-ae2b-00091a31506e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
