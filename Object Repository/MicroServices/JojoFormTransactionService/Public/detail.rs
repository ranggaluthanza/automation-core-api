<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>detail</name>
   <tag></tag>
   <elementGuidId>e649edfc-f9b9-4317-8818-ca182c9f1a5f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: 716\n}&quot;,
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
      <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJqb2pvbm9taWMtand0LXNlcnZpY2UiLCJpYXQiOjE1NzA2Mjc1NzUsImV4cCI6MTYwMjE2MzU3NSwic3ViIjoxMzQ1OCwic2Vzc19pZCI6Njc5MjAsInVzZXIiOnsiaWQiOjEzNDU4LCJlbWFpbCI6Impvam9xYUBtYWlsaW5hdG9yLmNvbSIsImNvbXBhbnlfaWQiOjQxNCwidXNlcl9jb21wYW55X2lkIjoyMzUzfSwic2Vzc2lvbl9zZXR0aW5nIjoxfQ.PrS7y3xGU1kDpvpaKfwEgn2UOyBEnVWm9dwQ6Vfy7tA</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}//detail</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.MicroServices</defaultValue>
      <description></description>
      <id>177d9be6-ffe0-476e-992a-9d05a3d4321d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>716</defaultValue>
      <description></description>
      <id>f6093d96-8488-4127-a928-f51fcd204001</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def id = request.getVariables().get('id')

if (id == null){
	WS.verifyResponseStatusCode(response, 500)
	WS.verifyElementPropertyValue(response, 'error', true)
}
else{
	WS.verifyResponseStatusCode(response, 200)
	WS.verifyElementPropertyValue(response, 'error', false)
	WS.verifyElementPropertyValue(response, 'message', 'Successfully get detail')
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
