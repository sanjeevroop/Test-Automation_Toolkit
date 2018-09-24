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
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://implementationnp.sdworx.co.uk/Account/Login?ReturnUrl=%2F')

WebUI.setText(findTestObject('Navigations/Navigation - Tax Groups/input_Email'), 'nadia.fareedun@sdworx.com')

WebUI.setText(findTestObject('Navigations/Navigation - Tax Groups/input_Password'), 'Password5!')

WebUI.click(findTestObject('Navigations/Navigation - Tax Groups/input_btn btn-default'))

WebUI.selectOptionByValue(findTestObject('Navigations/Navigation - Tax Groups/select_Please SelectOrg Test 1'), '5ff3bc22-7cb8-4dac-bddf-c14c8e62cdbb', 
    true)
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Tax Groups/a_Your Payroll Rules'))
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Tax Groups/a_Tax Groups'))
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Tax Groups/a_Add Tax Group'))

