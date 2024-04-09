<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_1                                      _97fbcd</name>
   <tag></tag>
   <elementGuidId>65159bdb-dd01-4709-bd84-2a25f87b07e3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#results</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='results']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>283d5a6d-f9bb-43dc-9414-b7c08033e83f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>results</value>
      <webElementGuid>d6bc4644-2dd5-4324-82d8-d216d097215b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>bg w l</value>
      <webElementGuid>bbf8dc5e-f91b-44bf-b52a-e2ab4424da2b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>	
	
     
        
	
	    
	      序號
	       
	      編輯
	      
	      刪除
	      救回
	      讀者姓名
	      讀者證號
	      讀者身份類別
	      所系單位
	      讀者狀態
	      發卡狀況
	      
	    
		
	      1
	        
		  
		     
		  
		  
		  
		     
		     
		      
		     
	      	    
		  
		      
	      	
		  
		    
		         
		         楊喬茹
		        
		    
		    
		  
		  
		  
		   
		   
		  
		  
		    57尊爵會員
		  
		  
		   台中總館 [LIB01]
		  
		  
		   有效讀者
		  
		  
		   
		  
		  
		
	
	
	 
	
    

	



	

		
		
						
			          		  
					   
  

//&lt;![CDATA[

  	var exMsg='頁碼錯誤，請重新輸入'
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == 'ro')
          Tapestry.submit_form('CautareRapida', 'langFirst');
      if (document.CautareRapida.languages.value == 'ru')
          Tapestry.submit_form('CautareRapida', 'langSecond');
      if (document.CautareRapida.languages.value == 'en')
          Tapestry.submit_form('CautareRapida', 'langThird');
      if (document.CautareRapida.languages.value == 'ch')
          Tapestry.submit_form('CautareRapida', 'langFourth');
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = '1';
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
							 
			          
			
					  
			
					
			
			
				
					
		   
             刪除   
			 回存 
			
			
			
			
			
			
			
			
			
			
			
				
				
			
		
	
		 
</value>
      <webElementGuid>adb27c56-9f3d-4c6c-9716-c3407a45f43e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;results&quot;)</value>
      <webElementGuid>52fd090f-87e9-499e-b8f3-4ba0b5a76286</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='results']</value>
      <webElementGuid>20fdf52c-dc1e-4157-835f-4f51c20ab124</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='searchForm']/div[6]</value>
      <webElementGuid>b6db849a-4be0-42a9-81c6-4373717c25e1</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='GO'])[1]/following::div[1]</value>
      <webElementGuid>d547be65-733f-4315-9dc5-1d84cb3e33e2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div[6]</value>
      <webElementGuid>47b0931a-ca55-4b30-9115-a21e02e00162</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'results' and (text() = concat(&quot;	
	
     
        
	
	    
	      序號
	       
	      編輯
	      
	      刪除
	      救回
	      讀者姓名
	      讀者證號
	      讀者身份類別
	      所系單位
	      讀者狀態
	      發卡狀況
	      
	    
		
	      1
	        
		  
		     
		  
		  
		  
		     
		     
		      
		     
	      	    
		  
		      
	      	
		  
		    
		         
		         楊喬茹
		        
		    
		    
		  
		  
		  
		   
		   
		  
		  
		    57尊爵會員
		  
		  
		   台中總館 [LIB01]
		  
		  
		   有效讀者
		  
		  
		   
		  
		  
		
	
	
	 
	
    

	



	

		
		
						
			          		  
					   
  

//&lt;![CDATA[

  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
							 
			          
			
					  
			
					
			
			
				
					
		   
             刪除   
			 回存 
			
			
			
			
			
			
			
			
			
			
			
				
				
			
		
	
		 
&quot;) or . = concat(&quot;	
	
     
        
	
	    
	      序號
	       
	      編輯
	      
	      刪除
	      救回
	      讀者姓名
	      讀者證號
	      讀者身份類別
	      所系單位
	      讀者狀態
	      發卡狀況
	      
	    
		
	      1
	        
		  
		     
		  
		  
		  
		     
		     
		      
		     
	      	    
		  
		      
	      	
		  
		    
		         
		         楊喬茹
		        
		    
		    
		  
		  
		  
		   
		   
		  
		  
		    57尊爵會員
		  
		  
		   台中總館 [LIB01]
		  
		  
		   有效讀者
		  
		  
		   
		  
		  
		
	
	
	 
	
    

	



	

		
		
						
			          		  
					   
  

//&lt;![CDATA[

  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
							 
			          
			
					  
			
					
			
			
				
					
		   
             刪除   
			 回存 
			
			
			
			
			
			
			
			
			
			
			
				
				
			
		
	
		 
&quot;))]</value>
      <webElementGuid>5616abbe-23c0-4a34-83b6-97ea2574da72</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
