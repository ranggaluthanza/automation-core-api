<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create</name>
   <tag></tag>
   <elementGuidId>23d3fe98-b244-4bc1-a690-1c24d1764cdd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: 0,\n    \&quot;name\&quot;: \&quot;Testing jojoQA\&quot;,\n    \&quot;description\&quot;: \&quot;\&quot;,\n    \&quot;smart_connect\&quot;: {\n        \&quot;is_use_category\&quot;: ${is_use_category},\n        \&quot;is_use_amount\&quot;: ${is_use_amount},\n        \&quot;product_types\&quot;: [\n            1,\n            2,\n            3\n        ]\n    },\n    \&quot;additional_info\&quot;: [\n        {\n            \&quot;id\&quot;: \&quot;\&quot;,\n            \&quot;order_id\&quot;: 1,\n            \&quot;is_mandatory\&quot;: true,\n            \&quot;is_backdate\&quot;: false,\n            \&quot;is_can_similar\&quot;: false,\n            \&quot;name\&quot;: \&quot;IPK\&quot;,\n            \&quot;type\&quot;: \&quot;number_decimal\&quot;,\n            \&quot;placeholder\&quot;: \&quot;\&quot;,\n            \&quot;percentage\&quot;: \&quot;\&quot;,\n            \&quot;source_id\&quot;: \&quot;\&quot;,\n            \&quot;groups\&quot;: [],\n            \&quot;is_delete\&quot;: false,\n            \&quot;is_editable\&quot;: true,\n            \&quot;case_id\&quot;: \&quot;\&quot;,\n            \&quot;is_can_access_gallery\&quot;: false,\n            \&quot;map_type\&quot;: \&quot;1\&quot;,\n            \&quot;camera_use_type\&quot;: \&quot;0\&quot;\n        },\n        {\n            \&quot;id\&quot;: \&quot;\&quot;,\n            \&quot;order_id\&quot;: 2,\n            \&quot;is_mandatory\&quot;: false,\n            \&quot;is_backdate\&quot;: false,\n            \&quot;is_can_similar\&quot;: false,\n            \&quot;name\&quot;: \&quot;Age\&quot;,\n            \&quot;type\&quot;: \&quot;text\&quot;,\n            \&quot;placeholder\&quot;: \&quot;\&quot;,\n            \&quot;percentage\&quot;: \&quot;\&quot;,\n            \&quot;source_id\&quot;: \&quot;\&quot;,\n            \&quot;groups\&quot;: [],\n            \&quot;is_delete\&quot;: false,\n            \&quot;is_editable\&quot;: true,\n            \&quot;case_id\&quot;: \&quot;\&quot;,\n            \&quot;is_can_access_gallery\&quot;: false,\n            \&quot;map_type\&quot;: \&quot;1\&quot;,\n            \&quot;camera_use_type\&quot;: \&quot;0\&quot;\n        }\n    ],\n    \&quot;is_delete\&quot;: false\n}&quot;,
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
   <restUrl>${url}/additional-info/form/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.MicroServicesSetupForm</defaultValue>
      <description></description>
      <id>40880883-644b-4ba6-a3c4-4785d20f375a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>'Expense Connect Form'</defaultValue>
      <description></description>
      <id>3b292328-f965-4778-a70d-7e4883cc4a81</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>7dcdba19-6e02-4e7c-9d99-f405915f0ab4</id>
      <masked>false</masked>
      <name>is_mandatory</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>599845bb-c1b0-432d-84cf-bd42d9198ff6</id>
      <masked>false</masked>
      <name>source_id</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>a52b4125-b177-4444-a0c6-a0de881edc56</id>
      <masked>false</masked>
      <name>is_back_date</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>14060207-e8e7-4a3a-b575-67d49b027552</id>
      <masked>false</masked>
      <name>is_can_similar</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>90645773-1854-4c7c-aa96-fe2a2d99aa74</id>
      <masked>false</masked>
      <name>is_delete</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>9a0da46b-a725-4496-9a67-a1a8e4985a43</id>
      <masked>false</masked>
      <name>is_editable</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>52b113da-7a46-4a9d-8dbd-dd00e4db57de</id>
      <masked>false</masked>
      <name>is_can_access_gallery</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>3fc08548-9982-4f9c-a04b-64229b35fc64</id>
      <masked>false</masked>
      <name>camera_use_type</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>c98df131-81f9-44f2-b955-70e044208fc8</id>
      <masked>false</masked>
      <name>map_type</name>
   </variables>
   <variables>
      <defaultValue>[]</defaultValue>
      <description></description>
      <id>c69e5918-09af-49e2-96e4-97914701deaa</id>
      <masked>false</masked>
      <name>available_list</name>
   </variables>
   <variables>
      <defaultValue>[]</defaultValue>
      <description></description>
      <id>94bf3e9f-9348-481e-83f8-348a9b465fad</id>
      <masked>false</masked>
      <name>multiple_list</name>
   </variables>
   <variables>
      <defaultValue>[('id') : '9512', ('name') : 'Transportation']</defaultValue>
      <description></description>
      <id>3c148b6c-56b9-41f9-8aff-b6441a727dfb</id>
      <masked>false</masked>
      <name>category</name>
   </variables>
   <variables>
      <defaultValue>400000</defaultValue>
      <description></description>
      <id>d20434a6-dd2c-4491-88dd-8ca1c60c9259</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>546b823a-1127-4c84-b98a-fd782845b653</id>
      <masked>false</masked>
      <name>is_use_amount</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>6508f8e2-0a97-413e-a240-30810a56b7d6</id>
      <masked>false</masked>
      <name>is_use_category</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println response.getResponseBodyContent()

def title 				 = request.getVariables().get('title')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
