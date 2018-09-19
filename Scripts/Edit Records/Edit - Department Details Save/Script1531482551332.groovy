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

//WebUI.openBrowser('')

//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3DfZB8KSYOkKaION_QEhM5QwklklLJP3WiBSEeIdjCmP5z9zgOu9f35044SVhQ8rHHuiwk4Z3gbozIYFkLK5WupPxPRrSPJ35XJYXeKPmWw-zAtrg7vtOVw5GnCPkendJEiT749XRbyqN4gGmCtujE-Q&nonce=636670793888948944.YWZjYWU2YzgtMmJkNC00OWM3LTg3MjQtM2ZiMDVhZmZkNTRlZTgxMWEwMTUtOTVhZi00YjI0LThhOGYtYjk5NzhjNjUxNmNi&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')

//WebUI.setText(findTestObject('Edit Records/Edit - Department Details Save/Page_Sign in to your account/input_loginfmt'), 
    //'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Edit Records/Edit - Department Details Save/Page_Sign in to your account/input_idSIButton9'))

//WebUI.click(findTestObject('Edit Records/Edit - Department Details Save/button_Your Setup'))

//WebUI.click(findTestObject('Edit Records/Edit - Department Details Save/a_Your Payroll'))

//WebUI.click(findTestObject('Edit Records/Edit - Department Details Save/a_Departments'))

//WebUI.selectOptionByValue(findTestObject('Edit Records/Edit - Department Details Save/select_181818 - Zing Zing19191'), 
    //'c252159e-8f76-44c3-81ae-5da3541da275', true)

WebUI.delay(2)

WebUI.setText(findTestObject('Edit Records/Edit - Department Details Save/input'), findTestData
	('Search Department').getValue(1, 1))

//'7777')

WebUI.delay(2)

WebUI.click(findTestObject('Edit Records/Edit - Department Details Save/a_Edit'))

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/Edit Records/Edit - Department Details Save/input_DepartmentNumber905e37e4'),
	findTestData('Edit Department Save').getValue(1, 1))

//WebUI.setText(findTestObject('Edit Records/Edit - Department Details Save/input_DepartmentDescriptionddc'), 
//findTestData('Edit Department Save').getValue(2, 1))

//WebUI.setText(findTestObject('Edit Records/Edit - Department Details Save/input_ReferenceCodeddc84887-18'), 
//findTestData('Edit Department Save').getValue(3, 1))

WebUI.delay(2)

WebUI.click(findTestObject('Edit Records/Edit - Department Details Save/button_Save'))

