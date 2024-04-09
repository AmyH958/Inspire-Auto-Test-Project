<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_DetailsdjConfig  baseRelativePathinspi_19211e</name>
   <tag></tag>
   <elementGuidId>6d139d70-356f-4f23-a1a7-553cb30d76ad</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//html</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
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
      <value>html</value>
      <webElementGuid>7f98b4e1-732e-4ef3-b55e-166f772949e5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding='UTF-8';

































&lt;!--
if (typeof window.event == 'undefined'){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == 'INPUT' &amp;&amp; test_type == 'TEXT') ||(test_var == 'INPUT' &amp;&amp; test_type == 'PASSWORD')|| test_var == 'TEXTAREA'){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == 'INPUT' &amp;&amp; test_type == 'TEXT') ||(test_var == 'INPUT' &amp;&amp; test_type == 'PASSWORD') || test_var == 'TEXTAREA'){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
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
dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
	Book Seting
 
		  
		  
		  
		    
			
			
		  
		


	
	      

 





	





		
			
				
					
						
							
								條碼號：
								
								 
							          							            
						         
							
						
						
							
								卷號：	
									
						    
						
							
								書目資訊：
								
					        
						
						
						    
				  	    
					
				
				
				
	



	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;newForm&quot;);

closeDialogComponent('CloseReminderDialog');
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById('ajaxLoaderDiv');
            	var processDiv = document.getElementById('pressingDiv');
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById('pressingDiv');
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent('TinreadErrorDialog');
closeDialogComponent('TinreadMessageDialog');

tapestry.form.focusField('TextField');});
// -->





/html[1]</value>
      <webElementGuid>e2c58a84-a89a-445d-bca5-48a9d598dda9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>2b569ed8-e9b7-4aba-bbd1-8bec238dca05</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>bf93dfc2-cae2-4c27-b1f8-a0de3566bb81</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;

































&lt;!--
if (typeof window.event == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)|| test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;) || test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
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
dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
	Book Seting
 
		  
		  
		  
		    
			
			
		  
		


	
	      

 





	





		
			
				
					
						
							
								條碼號：
								
								 
							          							            
						         
							
						
						
							
								卷號：	
									
						    
						
							
								書目資訊：
								
					        
						
						
						    
				  	    
					
				
				
				
	



	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;newForm&quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;CloseReminderDialog&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextField&quot; , &quot;'&quot; , &quot;);});
// -->





/html[1]&quot;) or . = concat(&quot;



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;

































&lt;!--
if (typeof window.event == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)|| test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;) || test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
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
dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
	Book Seting
 
		  
		  
		  
		    
			
			
		  
		


	
	      

 





	





		
			
				
					
						
							
								條碼號：
								
								 
							          							            
						         
							
						
						
							
								卷號：	
									
						    
						
							
								書目資訊：
								
					        
						
						
						    
				  	    
					
				
				
				
	



	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;newForm&quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;CloseReminderDialog&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextField&quot; , &quot;'&quot; , &quot;);});
// -->





/html[1]&quot;))]</value>
      <webElementGuid>5752467d-590d-4927-b251-5862e8ef308d</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
