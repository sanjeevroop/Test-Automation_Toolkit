import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable


WebUI.delay(2)

WebUI.setText(findTestObject('Edit Records/Edit - Department Details Cancel/input'), findTestData
	('Search Department').getValue(1, 1))

WebUI.delay(2)

WebUI.click(findTestObject('Edit Records/Edit - Department Details Cancel/a_Edit'))

WebUI.setText(findTestObject('Edit Records/Edit - Department Details Cancel/input_DepartmentNumber905e37e4' ), 
findTestData('Edit Department Cancel').getValue(1, 1))



//Comments:
//If tester want to use fields other than Department Number, then Department Description needs to be uncommented and
//also the object need to be updated......

//WebUI.setText(findTestObject('Edit Records/Edit - Department Details Cancel/input_DepartmentDescription905'), 
//findTestData('Edit Department Cancel').getValue(2, 1))



//Comments:
//If tester want to use fields other than Department Number, then Department Description needs to be uncommented and
//also the object need to be updated......

//WebUI.setText(findTestObject('Edit Records/Edit - Department Details Cancel/input_ReferenceCode905e37e4-c5'), 
 //findTestData('Edit Department Cancel').getValue(3, 1))

WebUI.delay(2)

WebUI.click(findTestObject('Edit Records/Edit - Department Details Cancel/button_Cancel'))

