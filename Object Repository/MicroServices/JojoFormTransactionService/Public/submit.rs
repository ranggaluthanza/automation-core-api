<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>submit</name>
   <tag></tag>
   <elementGuidId>3b07a39d-1b5a-438e-8520-5170540bc44e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ids\&quot; : ${ids}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJqb2pvbm9taWMtand0LXNlcnZpY2UiLCJpYXQiOjE1NjY3OTIyNTYsImV4cCI6MTU5ODMyODI1Niwic3ViIjoxMzQ1OCwic2Vzc19pZCI6MTg0NDEwLCJ1c2VyIjp7ImlkIjoxMzQ1OCwiZW1haWwiOiJqb2pvcWFAbWFpbGluYXRvci5jb20iLCJjb21wYW55X2lkIjo0MTQsInVzZXJfY29tcGFueV9pZCI6MjM1M30sInNlc3Npb25fc2V0dGluZyI6MX0.IoYGjwN63VLYwEiBSbHcMqB_gRTrWCKM1nrCOZq548k</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}//submit</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.MicroServices</defaultValue>
      <description></description>
      <id>837a48e3-1619-4fc4-a532-072923b1776b</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>[2760]</defaultValue>
      <description></description>
      <id>0f421333-dee5-4c6e-b7b9-2f33bbf9d488</id>
      <masked>false</masked>
      <name>ids</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println response.getResponseBodyContent()

def id = request.getVariables().get('ids')

if (id == null){
	WS.verifyResponseStatusCode(response, 500)
	WS.verifyElementPropertyValue(response, 'error', true)
}
else {
	WS.verifyResponseStatusCode(response, 200)
	WS.verifyElementPropertyValue(response, 'error', false)
	WS.verifyElementPropertyValue(response, 'message', 'Successfully get submit')
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
