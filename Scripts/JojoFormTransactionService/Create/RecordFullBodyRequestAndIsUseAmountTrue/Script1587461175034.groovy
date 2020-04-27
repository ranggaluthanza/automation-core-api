import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.jojonomic.katalon.database.mongodb.Helper as Helper
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.mongodb.DB as DB
import com.mongodb.DBCollection

import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable

def Slurper = new JsonSlurper()

createForm = WS.sendRequestAndVerify(findTestObject('MicroServices/JojoFormService/create', [('url') : GlobalVariable.MicroServicesSetupForm
            , ('title') : 'Expense Connect Form', ('is_mandatory') : true, ('source_id') : 0, ('is_back_date') : false, ('is_can_similar') : false
            , ('is_delete') : false, ('is_editable') : true, ('is_can_access_gallery') : true, ('camera_use_type') : 0, ('map_type') : 1
            , ('available_list') : [], ('multiple_list') : [], ('category') : [('id') : '9512', ('name') : 'Transportation']
            , ('amount') : 400000, ('is_use_amount') : true, ('is_use_category') : true]))

def setupForm = Slurper.parseText(createForm.getResponseBodyContent())

isUseAmount = setupForm.data.smart_connect.is_use_amount

mongoClient = Helper.connectToDatabase('admin', 'admin', 'Indonesia1')

DB db = mongoClient.getDB('digiform')

collection = db.getCollection('forms')

DBCollection cursor = collection.find(id : setupForm.data.id)
println cursor
